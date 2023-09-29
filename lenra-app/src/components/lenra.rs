use crate::from_value;

include!("../gen/components_lenra.rs");

impl Into<LenraComponent> for builder::Actionable {
    fn into(self) -> LenraComponent {
        LenraComponent::Actionable(Box::new(self.try_into().unwrap()))
    }
}

impl Into<LenraComponent> for builder::Button {
    fn into(self) -> LenraComponent {
        LenraComponent::Button(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Carousel {
    fn into(self) -> LenraComponent {
        LenraComponent::Carousel(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Checkbox {
    fn into(self) -> LenraComponent {
        LenraComponent::Checkbox(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Container {
    fn into(self) -> LenraComponent {
        LenraComponent::Container(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::DropdownButton {
    fn into(self) -> LenraComponent {
        LenraComponent::DropdownButton(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Flex {
    fn into(self) -> LenraComponent {
        LenraComponent::Flex(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Flexible {
    fn into(self) -> LenraComponent {
        LenraComponent::Flexible(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Form {
    fn into(self) -> LenraComponent {
        LenraComponent::Form(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Icon {
    fn into(self) -> LenraComponent {
        LenraComponent::Icon(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Image {
    fn into(self) -> LenraComponent {
        LenraComponent::Image(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Menu {
    fn into(self) -> LenraComponent {
        LenraComponent::Menu(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::MenuItem {
    fn into(self) -> LenraComponent {
        LenraComponent::MenuItem(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::OverlayEntry {
    fn into(self) -> LenraComponent {
        LenraComponent::OverlayEntry(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Radio {
    fn into(self) -> LenraComponent {
        LenraComponent::Radio(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Slider {
    fn into(self) -> LenraComponent {
        LenraComponent::Slider(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Stack {
    fn into(self) -> LenraComponent {
        LenraComponent::Stack(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::StatusSticker {
    fn into(self) -> LenraComponent {
        LenraComponent::StatusSticker(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Text {
    fn into(self) -> LenraComponent {
        LenraComponent::Text(self.try_into().unwrap())
    }
}

// impl Into<LenraComponent> for lenra_gen::builder::TextField {
//     fn into(self) -> LenraComponent {
//         LenraComponent::TextField(self.try_into().unwrap())
//     }
// }

impl Into<LenraComponent> for builder::Toggle {
    fn into(self) -> LenraComponent {
        LenraComponent::Toggle(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::View {
    fn into(self) -> LenraComponent {
        LenraComponent::View(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for builder::Wrap {
    fn into(self) -> LenraComponent {
        LenraComponent::Wrap(self.try_into().unwrap())
    }
}

impl StylesPadding {
    pub fn symmetric(vertical: u16, horizontal: u16) -> StylesPadding {
        StylesPadding {
            top: Some(vertical.into()),
            bottom: Some(vertical.into()),
            left: Some(horizontal.into()),
            right: Some(horizontal.into()),
        }
    }
}

from_value!(DefsProps);
from_value!(DataQuery);
from_value!(DataProjection);

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn simple_lenra_component() {
        let result: LenraComponent = flex(vec![text("Hello").into(), text("World").into()]).into();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            r#"{"children":[{"_type":"text","value":"Hello"},{"_type":"text","value":"World"}],"_type":"flex"}"#
        );
    }

    #[test]
    fn simple_icon() {
        let icon: Icon = icon(StylesIconName::AcUnit).try_into().unwrap();
        assert_eq!(
            serde_json::to_string(&icon).unwrap(),
            r#"{"_type":"icon","value":"ac_unit"}"#
        );
    }
}
