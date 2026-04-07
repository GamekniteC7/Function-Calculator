use crate::gui::Widgets;
use fltk::prelude::WidgetExt;

pub(crate) fn init_output_gui(mut widgets: Widgets) {
    widgets
        .win
        .set_size(widgets.win.w(), widgets.win.h() + (widgets.win.h() / 2));

    widgets.root_output.set_pos(10, widgets.win.h() - 10);
    widgets.root_output.show();
}
