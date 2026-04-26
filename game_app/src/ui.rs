#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl UiRect {
    pub fn contains(self, point: (f32, f32)) -> bool {
        point.0 >= self.x
            && point.0 <= self.x + self.w
            && point.1 >= self.y
            && point.1 <= self.y + self.h
    }
}

pub fn centered_button_stack(
    screen_width: f32,
    center_y: f32,
    button_count: usize,
    button_width: f32,
    button_height: f32,
    spacing: f32,
) -> Vec<UiRect> {
    let total_height =
        button_count as f32 * button_height + button_count.saturating_sub(1) as f32 * spacing;
    let start_y = center_y - total_height / 2.0;
    let start_x = (screen_width - button_width) / 2.0;

    (0..button_count)
        .map(|index| UiRect {
            x: start_x,
            y: start_y + index as f32 * (button_height + spacing),
            w: button_width,
            h: button_height,
        })
        .collect()
}

pub fn hit_test(rects: &[UiRect], point: (f32, f32)) -> Option<usize> {
    rects.iter().position(|rect| rect.contains(point))
}

pub fn wrap_selection(current: usize, item_count: usize, delta: isize) -> usize {
    if item_count == 0 {
        return 0;
    }

    (current as isize + delta).rem_euclid(item_count as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_stack_places_buttons_evenly() {
        let rects = centered_button_stack(800.0, 300.0, 3, 240.0, 60.0, 20.0);

        assert_eq!(rects.len(), 3);
        assert_eq!(
            rects[0],
            UiRect {
                x: 280.0,
                y: 190.0,
                w: 240.0,
                h: 60.0,
            }
        );
        assert_eq!(rects[1].y, 270.0);
        assert_eq!(rects[2].y, 350.0);
    }

    #[test]
    fn hit_test_returns_matching_button_index() {
        let rects = centered_button_stack(640.0, 240.0, 2, 200.0, 50.0, 10.0);

        assert_eq!(hit_test(&rects, (320.0, 215.0)), Some(0));
        assert_eq!(hit_test(&rects, (320.0, 280.0)), Some(1));
        assert_eq!(hit_test(&rects, (50.0, 50.0)), None);
    }

    #[test]
    fn wrap_selection_cycles_both_directions() {
        assert_eq!(wrap_selection(0, 3, -1), 2);
        assert_eq!(wrap_selection(2, 3, 1), 0);
        assert_eq!(wrap_selection(1, 3, 1), 2);
    }
}
