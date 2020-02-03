pub mod pass;

pub trait RouteResolver<V>
where
    V: Into<glm::Vec2>,
{
    fn route_resolve<I>(&self, route: (V, V), others: I) -> bool
    where
        I: Iterator<Item = V>;
}
