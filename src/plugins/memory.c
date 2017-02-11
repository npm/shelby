#include <uv.h>
#include <stdio.h>
#include <string.h>
#include <forza.h>

static uv_loop_t* loop;
static uv_timer_t memory_timer;
static char meminfo_buf[4096];
static uv_fs_t meminfo_open_req, meminfo_read_req, meminfo_close_req;

void memory__meminfo_read_cb(uv_fs_t* req) {
  char name[255];
  unsigned long int value, mem_total, mem_free, cached, buffers;
  char* token = NULL;

  if (req->result == -1) {
    fprintf(stderr, "error reading /proc/meminfo: %s\n", uv_strerror(uv_last_error(loop)));
  }

  uv_fs_req_cleanup(req);
  uv_fs_close(loop, &meminfo_close_req, meminfo_open_req.result, NULL);

  // Parse /proc/meminfo we just read. We're looking for total amount of used
  // memory without cached and buffers, which the operating system will reclaim/flush
  // as needed.
  // TODO: what about quantum memory?

  token = strtok(meminfo_buf, "\n");
  while (token != NULL) {
    sscanf(token, "%s %8lu", name, &value);

    if (strcmp(name, "MemTotal:") == 0) mem_total = value;
    if (strcmp(name, "MemFree:") == 0) mem_free = value;
    if (strcmp(name, "Cached:") == 0) cached = value;
    if (strcmp(name, "Buffers:") == 0) buffers = value;

    token = strtok(NULL, "\n");
  }

  // Exactly mimic Nagios' memory plugin.
  unsigned long int mem_used = mem_total - mem_free - cached - buffers;

  forza_metric_t* metric = forza_new_metric();
  metric->service = "memory";
  metric->metric = (double) mem_used / (double) mem_total;
  forza_send(metric);
  forza_free_metric(metric);
}

void memory__meminfo_open_cb(uv_fs_t* req) {
  if (req->result == -1) {
    fprintf(stderr, "error opening /proc/meminfo: %s\n", uv_strerror(uv_last_error(loop)));
  }

  uv_fs_req_cleanup(req);

  uv_fs_read(
    loop,
    &meminfo_read_req,
    req->result,
    meminfo_buf,
    sizeof(meminfo_buf),
    0,
    memory__meminfo_read_cb
  );
}

void memory__read_meminfo() {
  int r = uv_fs_open(loop, &meminfo_open_req, "/proc/meminfo", O_RDONLY, 0, memory__meminfo_open_cb);
  if (r == -1) {
    fprintf(stderr, "error opening /proc/meminfo: %s\n", uv_strerror(uv_last_error(loop)));
  }
}

void memory__send_usage(uv_timer_t *timer, int status) {
#ifdef DEBUG
  printf("memory usage timer fired, status %d\n", status);
#endif
  memory__read_meminfo();
}

void memory__process_exit_cb(int exit_status, int term_singal) {
  uv_timer_stop(&memory_timer);
}

int memory_init(forza_plugin_t* plugin) {
  loop = uv_default_loop();

  plugin->process_exit_cb = memory__process_exit_cb;

  uv_timer_init(loop, &memory_timer);
  uv_timer_start(&memory_timer, memory__send_usage, 0, 5000);

  return 0;
}
