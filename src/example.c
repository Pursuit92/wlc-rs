// For more functional example see example/example.c

#include <stdlib.h>
#include <stdio.h>
#include <wlc/wlc.h>

static bool
view_created(wlc_handle view)
{
  printf("created\n");
  wlc_view_bring_to_front(view);
  wlc_view_focus(view);
  return true;
}

static void
view_focus(wlc_handle view, bool focus)
{
  printf("focused\n");
  wlc_view_set_state(view, WLC_BIT_ACTIVATED, focus);
}

int
main(int argc, char *argv[])
{
  static struct wlc_interface interface = {
    .view = {
      .created = view_created,
      .focus = view_focus,
    },
  };

  if (!wlc_init(&interface, argc, argv))
    return EXIT_FAILURE;

  wlc_run();
  return EXIT_SUCCESS;
}
