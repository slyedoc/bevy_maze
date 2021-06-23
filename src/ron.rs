pub const visuals: Visuals = Visuals {
    dark_mode: true,
    override_text_color: None,
    widgets: Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: Color32([30, 30, 30, 255]),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32([65, 65, 65, 255]),
            },
            corner_radius: 4.0,
            fg_stroke: Stroke {
                width: 1.0,
                color: Color32([160, 160, 160, 255]),
            },
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            bg_fill: Color32([70, 70, 70, 255]),
            bg_stroke: Stroke {
                width: 0.0,
                color: Color32([0, 0, 0, 0]),
            },
            corner_radius: 4.0,
            fg_stroke: Stroke {
                width: 1.0,
                color: Color32([200, 200, 200, 255]),
            },
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            bg_fill: Color32([80, 80, 80, 255]),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32([150, 150, 150, 255]),
            },
            corner_radius: 4.0,
            fg_stroke: Stroke {
                width: 1.5,
                color: Color32([240, 240, 240, 255]),
            },
            expansion: 1.0,
        },
        active: WidgetVisuals {
            bg_fill: Color32([90, 90, 90, 255]),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32([255, 255, 255, 255]),
            },
            corner_radius: 4.0,
            fg_stroke: Stroke {
                width: 2.0,
                color: Color32([255, 255, 255, 255]),
            },
            expansion: 2.0,
        },
    },
    selection: Selection {
        bg_fill: Color32([0, 92, 128, 255]),
        stroke: Stroke {
            width: 1.0,
            color: Color32([192, 222, 255, 255]),
        },
    },
    extreme_bg_color: Color32([10, 10, 10, 255]),
    hyperlink_color: Color32([90, 170, 255, 255]),
    code_bg_color: Color32([64, 64, 64, 255]),
    window_corner_radius: 10.0,
    window_shadow: Shadow {
        extrusion: 32.0,
        color: Color32([0, 0, 0, 96]),
    },
    resize_corner_size: 9.2,
    text_cursor_width: 2.0,
    text_cursor_preview: false,
    clip_rect_margin: 3.0,
};
