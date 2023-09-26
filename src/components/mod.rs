use self::lenra_gen::LenraComponent;

pub mod lenra_gen;
pub mod listener_gen;

impl Into<LenraComponent> for lenra_gen::builder::Actionable {
    fn into(self) -> LenraComponent {
        LenraComponent::Actionable(Box::new(self.try_into().unwrap()))
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Button {
    fn into(self) -> LenraComponent {
        LenraComponent::Button(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Carousel {
    fn into(self) -> LenraComponent {
        LenraComponent::Carousel(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Checkbox {
    fn into(self) -> LenraComponent {
        LenraComponent::Checkbox(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Container {
    fn into(self) -> LenraComponent {
        LenraComponent::Container(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::DropdownButton {
    fn into(self) -> LenraComponent {
        LenraComponent::DropdownButton(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Flex {
    fn into(self) -> LenraComponent {
        LenraComponent::Flex(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Flexible {
    fn into(self) -> LenraComponent {
        LenraComponent::Flexible(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Form {
    fn into(self) -> LenraComponent {
        LenraComponent::Form(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Icon {
    fn into(self) -> LenraComponent {
        LenraComponent::Icon(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Image {
    fn into(self) -> LenraComponent {
        LenraComponent::Image(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Menu {
    fn into(self) -> LenraComponent {
        LenraComponent::Menu(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::MenuItem {
    fn into(self) -> LenraComponent {
        LenraComponent::MenuItem(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::OverlayEntry {
    fn into(self) -> LenraComponent {
        LenraComponent::OverlayEntry(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Radio {
    fn into(self) -> LenraComponent {
        LenraComponent::Radio(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Slider {
    fn into(self) -> LenraComponent {
        LenraComponent::Slider(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Stack {
    fn into(self) -> LenraComponent {
        LenraComponent::Stack(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::StatusSticker {
    fn into(self) -> LenraComponent {
        LenraComponent::StatusSticker(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Text {
    fn into(self) -> LenraComponent {
        LenraComponent::Text(self.try_into().unwrap())
    }
}

// impl Into<LenraComponent> for lenra_gen::builder::TextField {
//     fn into(self) -> LenraComponent {
//         LenraComponent::TextField(self.try_into().unwrap())
//     }
// }

impl Into<LenraComponent> for lenra_gen::builder::Toggle {
    fn into(self) -> LenraComponent {
        LenraComponent::Toggle(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::View {
    fn into(self) -> LenraComponent {
        LenraComponent::View(self.try_into().unwrap())
    }
}

impl Into<LenraComponent> for lenra_gen::builder::Wrap {
    fn into(self) -> LenraComponent {
        LenraComponent::Wrap(self.try_into().unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::lenra_gen::*;

    #[test]
    fn simple_lenra_component() {
        let result: LenraComponent = flex(vec![
            text("Hello".into()).into(),
            text("World".into()).into(),
        ])
        .into();
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
