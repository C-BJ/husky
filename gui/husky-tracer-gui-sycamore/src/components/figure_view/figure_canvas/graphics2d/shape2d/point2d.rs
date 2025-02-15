use super::*;

#[derive(Prop)]
pub struct Point2dProps<'a> {
    point: &'a Point2dData,
}

#[component]
pub fn Point2d<'a, G: Html>(scope: Scope<'a>, props: Point2dProps<'a>) -> View<G> {
    view! {
        scope,
        circle (
            cx={props.point.x},
            cy={props.point.y},
            r=0.1,
            fill="red"
        )
    }
}
