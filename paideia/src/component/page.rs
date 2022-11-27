use katalogos::IntoIterRef;

use super::{
    asset::AssetComponent,
    section::SectionComponent,
    BlockComponent,
    Component,
    ComponentKind,
    InlineComponent,
};
use crate::render::{Context, Html, Markdown, Render, Renderer, Text};
use std::{
    cmp::Ordering,
    fmt::{self, Write},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PageComponent;

impl ComponentKind for PageComponent {}

pub struct Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    pub title: String,
    pub assets: A,
    pub body: B,
    pub children: L,
}

impl<A, B, L> fmt::Debug for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_struct("UnorderedList");
        debug_fmtr.field("title", &self.title).field("body", &self.body);
        for (i, element) in self.assets.iter().enumerate() {
            debug_fmtr.field(&format!("asset[{}]", i), &element);
        }
        for (i, element) in self.children.iter().enumerate() {
            debug_fmtr.field(&format!("children[{}]", i), &element);
        }
        debug_fmtr.finish()
    }
}

impl<A, B, L> Clone for Page<A, B, L>
where
    A: IntoIterRef + Clone,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent> + Clone,
    L: IntoIterRef + Clone,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            assets: self.assets.clone(),
            body: self.body.clone(),
            children: self.children.clone(),
        }
    }
}

impl<A, B, L> PartialEq for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent> + PartialEq,
    B: Component<Kind = BlockComponent> + PartialEq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.assets.iter().eq(other.assets.iter())
            && self.body == other.body
            && self.children.iter().eq(other.children.iter())
    }
}

impl<A, B, L> Eq for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent> + Eq,
    B: Component<Kind = BlockComponent> + Eq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Eq,
{
}

impl<A, B, L> PartialOrd for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent> + PartialOrd,
    B: Component<Kind = BlockComponent> + PartialOrd,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self
            .title
            .partial_cmp(&other.title)?
            .then(self.assets.iter().partial_cmp(other.assets.iter())?)
            .then(self.body.partial_cmp(&other.body)?)
            .then(self.children.iter().partial_cmp(other.children.iter())?);
        Some(ordering)
    }
}

impl<A, B, L> Ord for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent> + Ord,
    B: Component<Kind = BlockComponent> + Ord,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.title
            .cmp(&other.title)
            .then_with(|| self.assets.iter().cmp(other.assets.iter()))
            .then_with(|| self.body.cmp(&other.body))
            .then_with(|| self.children.iter().cmp(other.children.iter()))
    }
}

impl<A, B, L> Hash for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent> + Hash,
    B: Component<Kind = BlockComponent> + Hash,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.title.hash(state);
        for (i, asset) in self.assets.iter().enumerate() {
            i.hash(state);
            asset.hash(state);
        }
        self.body.hash(state);
        for (i, child) in self.children.iter().enumerate() {
            i.hash(state);
            child.hash(state);
        }
    }
}
impl<A, B, L> Default for Page<A, B, L>
where
    A: IntoIterRef + Default,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent> + Default,
    L: IntoIterRef + Default,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    fn default() -> Self {
        Self {
            title: String::default(),
            assets: A::default(),
            body: B::default(),
            children: L::default(),
        }
    }
}

impl<A, B, L> Component for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    type Kind = PageComponent;
}

impl<A, B, L> Render<Html> for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Render<Html, Kind = AssetComponent>,
    B: Render<Html, Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Html, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><meta \
             name=\"viewport\" content=\"width=device-width, \
             initial-scale=1.0\">",
        )?;
        for asset in self.assets.iter() {
            asset.render(renderer, ctx.with_kind(&AssetComponent))?;
        }
        renderer.write_str("<title>")?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(
            "</title></head><body><div class=\"paideia-page-wrapper\" \
             id=\"paideia-page-root\"><h1 class=\"paideia-title\"><a \
             href=\"#paideia-page-root\">",
        )?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        write!(renderer, "</a></h1><div class=\"paideia-body\">")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        renderer.write_str("</div><div class=\"paideia-children\">")?;
        for child in self.children.iter() {
            child.render(renderer, ctx.with_kind(&SectionComponent))?;
        }
        renderer.write_str("</div></div></body></html>")?;
        Ok(())
    }
}

impl<A, B, L> Render<Markdown> for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Render<Markdown, Kind = AssetComponent>,
    B: Render<Markdown, Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Markdown, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("# ")?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in self.children.iter() {
            child.render(renderer, ctx.with_kind(&SectionComponent))?;
        }
        Ok(())
    }
}

impl<A, B, L> Render<Text> for Page<A, B, L>
where
    A: IntoIterRef,
    <A as IntoIterRef>::Item: Render<Text, Kind = AssetComponent>,
    B: Render<Text, Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Text, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in self.children.iter() {
            child.render(renderer, ctx.with_kind(&SectionComponent))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use katalogos::harray;

    use super::{Page, PageComponent};
    use crate::{
        component::{
            asset::{Script, Stylesheet},
            block::text::Paragraph,
            section::Section,
        },
        location::{Id, InternalPath, Location},
        render::{
            html::test::validate_html_document,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn page_without_assets_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Page {
                title: String::from("Hello"),
                assets: harray![],
                body: Paragraph("World!"),
                children: harray![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &PageComponent),
        )
        .to_string();

        validate_html_document(&rendered).unwrap();
    }

    #[test]
    fn page_with_assets_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Page {
                title: String::from("Hello"),
                assets: harray![
                    Stylesheet {
                        location: Location::internal("styles/main.css"),
                    },
                    Script { location: Location::internal("js/main.js") }
                ],
                body: Paragraph("World!"),
                children: harray![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &PageComponent),
        )
        .to_string();

        validate_html_document(&rendered).unwrap();
    }

    #[test]
    fn page_with_children_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Page {
                title: String::from("Hello"),
                assets: harray![Stylesheet {
                    location: Location::internal("styles/main.css"),
                }],
                body: Paragraph("World, aaaa!"),
                children: harray![
                    Section {
                        title: "Hey",
                        id: None,
                        body: Paragraph("Hey!"),
                        children: harray![],
                    },
                    Section {
                        title: "Good",
                        id: Some(Id::new("good").unwrap()),
                        body: Paragraph("Afternoon!"),
                        children: harray![Section {
                            title: "By",
                            id: None,
                            body: Paragraph("Bye!"),
                            children: harray![],
                        }],
                    },
                    Section {
                        title: "Hay",
                        id: None,
                        body: Paragraph("Bay!"),
                        children: harray![],
                    },
                ],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &PageComponent),
        )
        .to_string();

        validate_html_document(&rendered).unwrap();
    }
}
