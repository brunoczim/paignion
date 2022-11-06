use super::InlineComponent;
use crate::{
    component::Component,
    location::Location,
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Copy)]
pub struct Bold<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Bold<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Bold<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<span class=\"paideia-bold\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Bold<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("**")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("**")?;
        Ok(())
    }
}

impl<C> Render<Text> for Bold<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Italic<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Italic<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Italic<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<span class=\"paideia-bold\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Italic<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("_")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("_")?;
        Ok(())
    }
}

impl<C> Render<Text> for Italic<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Preformatted<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Preformatted<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Preformatted<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<span class=\"paideia-preformatted\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Preformatted<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<pre>")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</pre>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Preformatted<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link<C>
where
    C: Component<Kind = InlineComponent>,
{
    pub target: C,
    pub location: Location,
}

impl<C> Component for Link<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Link<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<a href=\"")?;
        self.location.render(renderer, ctx)?;
        renderer.write_str("\">")?;
        self.target.render(renderer, ctx)?;
        renderer.write_str("</a>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Link<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.target.render(renderer, ctx)?;
        renderer.write_str("](")?;
        self.location.render(renderer, ctx)?;
        renderer.write_str(")")?;
        Ok(())
    }
}

impl<C> Render<Text> for Link<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.target.render(renderer, ctx)
    }
}