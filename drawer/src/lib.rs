pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use plotters::prelude::*;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_plotters() -> Result<(), Box<dyn std::error::Error>>{
        let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("散点图", ("sans-serif", 30))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..10, 0..10)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            PointSeries::of_element(
                [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8), (9, 9)],
                5,
                &BLUE,
                &|c, s, st| {
                    return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                },
            ),
        )?;

        Ok(())
    }
}
