use serde::{Deserialize, Serialize};
///Element of type Actionable
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Actionable {
    pub child: LenraComponent,
    ///The listener to be called when the actionable is double pressed.
    #[serde(
        rename = "onDoublePressed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_double_pressed: Option<Listener>,
    ///The listener to be called when the actionable is hovered and when the mouse exits the hoverable area.
    #[serde(rename = "onHovered", default, skip_serializing_if = "Option::is_none")]
    pub on_hovered: Option<Listener>,
    ///The listener to be called when the actionable is long pressed.
    #[serde(rename = "onLongPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_long_pressed: Option<Listener>,
    ///The listener to be called when the actionable is pressed.
    #[serde(rename = "onPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Listener>,
    ///The listener to be called when the actionable is pressed inside and released outside of the actionable, causing it to cancel the press action.
    #[serde(
        rename = "onPressedCancel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_pressed_cancel: Option<Listener>,
    ///Whether the actionable can submit a form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit: Option<bool>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Actionable> for Actionable {
    fn from(value: &Actionable) -> Self {
        value.clone()
    }
}
impl Actionable {
    pub fn builder() -> builder::Actionable {
        builder::Actionable::default()
    }
}
///Element of type Button
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    ///The button is disabled if true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "leftIcon", default, skip_serializing_if = "Option::is_none")]
    pub left_icon: Option<Icon>,
    #[serde(rename = "mainStyle", default, skip_serializing_if = "Option::is_none")]
    pub main_style: Option<StylesStyle>,
    #[serde(rename = "onPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Listener>,
    #[serde(rename = "rightIcon", default, skip_serializing_if = "Option::is_none")]
    pub right_icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<StylesSize>,
    ///Whether the button is a submit button for a form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit: Option<bool>,
    ///The value of the text inside the button
    pub text: String,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Button> for Button {
    fn from(value: &Button) -> Self {
        value.clone()
    }
}
impl Button {
    pub fn builder() -> builder::Button {
        builder::Button::default()
    }
}
///Element of type Carousel
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Carousel {
    ///The children
    pub children: Vec<LenraComponent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<StylesCarouselOptions>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Carousel> for Carousel {
    fn from(value: &Carousel) -> Self {
        value.clone()
    }
}
impl Carousel {
    pub fn builder() -> builder::Carousel {
        builder::Carousel::default()
    }
}
///Element of type Checkbox
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Checkbox {
    ///Whether the checkbox is focused initially.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autofocus: Option<bool>,
    #[serde(
        rename = "materialTapTargetSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub material_tap_target_size: Option<StylesMaterialTapTargetSize>,
    ///The name that will be used in the form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "onPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Listener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StylesCheckboxStyle>,
    ///Whether the checkbox can have 3 states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tristate: Option<bool>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    ///The default state of the checkbox
    pub value: bool,
}
impl From<&Checkbox> for Checkbox {
    fn from(value: &Checkbox) -> Self {
        value.clone()
    }
}
impl Checkbox {
    pub fn builder() -> builder::Checkbox {
        builder::Checkbox::default()
    }
}
///Element of type container
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Container {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<StylesAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<StylesBorder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child: Option<Box<LenraComponent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<StylesBoxConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decoration: Option<StylesBoxDecoration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<StylesPadding>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Container> for Container {
    fn from(value: &Container) -> Self {
        value.clone()
    }
}
impl Container {
    pub fn builder() -> builder::Container {
        builder::Container::default()
    }
}
///Mongo data query projection
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataProjection(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DataProjection {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DataProjection> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DataProjection) -> Self {
        value.0
    }
}
impl From<&DataProjection> for DataProjection {
    fn from(value: &DataProjection) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DataProjection {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
///Mongo data query
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataQuery(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DataQuery {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DataQuery> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DataQuery) -> Self {
        value.0
    }
}
impl From<&DataQuery> for DataQuery {
    fn from(value: &DataQuery) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DataQuery {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
///Parameters passed to the listener
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefsProps(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DefsProps {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DefsProps> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DefsProps) -> Self {
        value.0
    }
}
impl From<&DefsProps> for DefsProps {
    fn from(value: &DefsProps) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DefsProps {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
///Element of type Dropdown Button
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DropdownButton {
    pub child: Box<LenraComponent>,
    ///If true, the dropdown button is disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(rename = "mainStyle", default, skip_serializing_if = "Option::is_none")]
    pub main_style: Option<StylesStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<StylesSize>,
    ///The text of the dropdown button
    pub text: String,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&DropdownButton> for DropdownButton {
    fn from(value: &DropdownButton) -> Self {
        value.clone()
    }
}
impl DropdownButton {
    pub fn builder() -> builder::DropdownButton {
        builder::DropdownButton::default()
    }
}
///Element of type Flex
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Flex {
    ///The children
    pub children: Vec<LenraComponent>,
    ///The alignment along the cross axis
    #[serde(
        rename = "crossAxisAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_axis_alignment: Option<FlexCrossAxisAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<StylesDirection>,
    ///if true the flex will fill the main axis. Otherwise it will take the children size.
    #[serde(rename = "fillParent", default, skip_serializing_if = "Option::is_none")]
    pub fill_parent: Option<bool>,
    #[serde(
        rename = "horizontalDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_direction: Option<StylesTextDirection>,
    ///The alignment along the main axis
    #[serde(
        rename = "mainAxisAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub main_axis_alignment: Option<FlexMainAxisAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<StylesPadding>,
    ///If true the flex will scroll if there is too many item in the Main Axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f64>,
    #[serde(rename = "textBaseline", default, skip_serializing_if = "Option::is_none")]
    pub text_baseline: Option<StylesTextBaseline>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    #[serde(
        rename = "verticalDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_direction: Option<StylesVerticalDirection>,
}
impl From<&Flex> for Flex {
    fn from(value: &Flex) -> Self {
        value.clone()
    }
}
impl Flex {
    pub fn builder() -> builder::Flex {
        builder::Flex::default()
    }
}
///The alignment along the cross axis
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum FlexCrossAxisAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "stretch")]
    Stretch,
    #[serde(rename = "baseline")]
    Baseline,
}
impl From<&FlexCrossAxisAlignment> for FlexCrossAxisAlignment {
    fn from(value: &FlexCrossAxisAlignment) -> Self {
        value.clone()
    }
}
impl ToString for FlexCrossAxisAlignment {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
            Self::Center => "center".to_string(),
            Self::Stretch => "stretch".to_string(),
            Self::Baseline => "baseline".to_string(),
        }
    }
}
impl std::str::FromStr for FlexCrossAxisAlignment {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            "baseline" => Ok(Self::Baseline),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for FlexCrossAxisAlignment {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlexCrossAxisAlignment {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlexCrossAxisAlignment {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The alignment along the main axis
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum FlexMainAxisAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "spaceBetween")]
    SpaceBetween,
    #[serde(rename = "spaceAround")]
    SpaceAround,
    #[serde(rename = "spaceEvenly")]
    SpaceEvenly,
}
impl From<&FlexMainAxisAlignment> for FlexMainAxisAlignment {
    fn from(value: &FlexMainAxisAlignment) -> Self {
        value.clone()
    }
}
impl ToString for FlexMainAxisAlignment {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
            Self::Center => "center".to_string(),
            Self::SpaceBetween => "spaceBetween".to_string(),
            Self::SpaceAround => "spaceAround".to_string(),
            Self::SpaceEvenly => "spaceEvenly".to_string(),
        }
    }
}
impl std::str::FromStr for FlexMainAxisAlignment {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "spaceBetween" => Ok(Self::SpaceBetween),
            "spaceAround" => Ok(Self::SpaceAround),
            "spaceEvenly" => Ok(Self::SpaceEvenly),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for FlexMainAxisAlignment {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlexMainAxisAlignment {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlexMainAxisAlignment {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type Flexible
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Flexible {
    pub child: Box<LenraComponent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit: Option<StylesFlexFit>,
    ///How a flexible child is inscribed into the available space.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex: Option<i64>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Flexible> for Flexible {
    fn from(value: &Flexible) -> Self {
        value.clone()
    }
}
impl Flexible {
    pub fn builder() -> builder::Flexible {
        builder::Flexible::default()
    }
}
///Element of type Form
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Form {
    pub child: Box<LenraComponent>,
    ///Callback when the user submits the form.
    #[serde(rename = "onSubmit", default, skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<Listener>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Form> for Form {
    fn from(value: &Form) -> Self {
        value.clone()
    }
}
impl Form {
    pub fn builder() -> builder::Form {
        builder::Form::default()
    }
}
///The Icon to use
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    ///The color of the Icon. If not set or null, the color is inherited from the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<StylesColor>,
    ///The semantic label for the Icon. This will be announced when using accessibility mode.
    #[serde(rename = "semanticLabel", default, skip_serializing_if = "Option::is_none")]
    pub semantic_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<IconDefinitionsIconStyle>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    ///The value of the Icon
    pub value: StylesIconName,
}
impl From<&Icon> for Icon {
    fn from(value: &Icon) -> Self {
        value.clone()
    }
}
impl Icon {
    pub fn builder() -> builder::Icon {
        builder::Icon::default()
    }
}
///The style of the Icon
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum IconDefinitionsIconStyle {
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "sharp")]
    Sharp,
    #[serde(rename = "rounded")]
    Rounded,
    #[serde(rename = "outlined")]
    Outlined,
}
impl From<&IconDefinitionsIconStyle> for IconDefinitionsIconStyle {
    fn from(value: &IconDefinitionsIconStyle) -> Self {
        value.clone()
    }
}
impl ToString for IconDefinitionsIconStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::Filled => "filled".to_string(),
            Self::Sharp => "sharp".to_string(),
            Self::Rounded => "rounded".to_string(),
            Self::Outlined => "outlined".to_string(),
        }
    }
}
impl std::str::FromStr for IconDefinitionsIconStyle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "filled" => Ok(Self::Filled),
            "sharp" => Ok(Self::Sharp),
            "rounded" => Ok(Self::Rounded),
            "outlined" => Ok(Self::Outlined),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IconDefinitionsIconStyle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IconDefinitionsIconStyle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IconDefinitionsIconStyle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type Image
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    ///How to align the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<StylesAlignment>,
    ///The center slice for a nine-patch image.
    #[serde(rename = "centerSlice", default, skip_serializing_if = "Option::is_none")]
    pub center_slice: Option<StylesRect>,
    ///The error placeholder when the image encounters an error during loading.
    #[serde(
        rename = "errorPlaceholder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_placeholder: Option<Box<LenraComponent>>,
    ///Whether to exclude this image from semantics.
    #[serde(
        rename = "excludeFromSemantics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_from_semantics: Option<bool>,
    ///The quality of the image.
    #[serde(rename = "filterQuality", default, skip_serializing_if = "Option::is_none")]
    pub filter_quality: Option<StylesFilterQuality>,
    ///How the image should fit the parent box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit: Option<StylesBoxFit>,
    ///A placeholder to display while the image is loading or to add effects to the image.
    #[serde(
        rename = "framePlaceholder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_placeholder: Option<Box<LenraComponent>>,
    ///Whether the old image (true) or nothing (false) is shown when the image provider changes.
    #[serde(
        rename = "gaplessPlayback",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gapless_playback: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    ///Whether to paint the image with anti-aliasing.
    #[serde(rename = "isAntiAlias", default, skip_serializing_if = "Option::is_none")]
    pub is_anti_alias: Option<bool>,
    ///A placeholder to display while the image is still loading.
    #[serde(
        rename = "loadingPlaceholder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loading_placeholder: Option<Box<LenraComponent>>,
    ///How the image should be repeated accross the bounds not covered by the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat: Option<StylesImageRepeat>,
    ///A semantic description of the image. This is used for TalkBack on Android and VoiceOver on IOS.
    #[serde(rename = "semanticLabel", default, skip_serializing_if = "Option::is_none")]
    pub semantic_label: Option<String>,
    ///The URL to the image. Will fetch the application's image if the URL does not start with `http(s)://`.
    pub src: String,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}
impl From<&Image> for Image {
    fn from(value: &Image) -> Self {
        value.clone()
    }
}
impl Image {
    pub fn builder() -> builder::Image {
        builder::Image::default()
    }
}
///Any component
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LenraComponent {
    Actionable(Box<Actionable>),
    Button(Button),
    Carousel(Carousel),
    Checkbox(Checkbox),
    Container(Container),
    DropdownButton(DropdownButton),
    Flex(Flex),
    Flexible(Flexible),
    Form(Form),
    Icon(Icon),
    Image(Image),
    Menu(Menu),
    MenuItem(MenuItem),
    OverlayEntry(OverlayEntry),
    Radio(Radio),
    Slider(Slider),
    Stack(Stack),
    StatusSticker(StatusSticker),
    Text(Text),
    Textfield(Textfield),
    Toggle(Toggle),
    View(View),
    Wrap(Wrap),
}
impl From<&LenraComponent> for LenraComponent {
    fn from(value: &LenraComponent) -> Self {
        value.clone()
    }
}
impl From<Box<Actionable>> for LenraComponent {
    fn from(value: Box<Actionable>) -> Self {
        Self::Actionable(value)
    }
}
impl From<Button> for LenraComponent {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}
impl From<Carousel> for LenraComponent {
    fn from(value: Carousel) -> Self {
        Self::Carousel(value)
    }
}
impl From<Checkbox> for LenraComponent {
    fn from(value: Checkbox) -> Self {
        Self::Checkbox(value)
    }
}
impl From<Container> for LenraComponent {
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}
impl From<DropdownButton> for LenraComponent {
    fn from(value: DropdownButton) -> Self {
        Self::DropdownButton(value)
    }
}
impl From<Flex> for LenraComponent {
    fn from(value: Flex) -> Self {
        Self::Flex(value)
    }
}
impl From<Flexible> for LenraComponent {
    fn from(value: Flexible) -> Self {
        Self::Flexible(value)
    }
}
impl From<Form> for LenraComponent {
    fn from(value: Form) -> Self {
        Self::Form(value)
    }
}
impl From<Icon> for LenraComponent {
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}
impl From<Image> for LenraComponent {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}
impl From<Menu> for LenraComponent {
    fn from(value: Menu) -> Self {
        Self::Menu(value)
    }
}
impl From<MenuItem> for LenraComponent {
    fn from(value: MenuItem) -> Self {
        Self::MenuItem(value)
    }
}
impl From<OverlayEntry> for LenraComponent {
    fn from(value: OverlayEntry) -> Self {
        Self::OverlayEntry(value)
    }
}
impl From<Radio> for LenraComponent {
    fn from(value: Radio) -> Self {
        Self::Radio(value)
    }
}
impl From<Slider> for LenraComponent {
    fn from(value: Slider) -> Self {
        Self::Slider(value)
    }
}
impl From<Stack> for LenraComponent {
    fn from(value: Stack) -> Self {
        Self::Stack(value)
    }
}
impl From<StatusSticker> for LenraComponent {
    fn from(value: StatusSticker) -> Self {
        Self::StatusSticker(value)
    }
}
impl From<Text> for LenraComponent {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}
impl From<Textfield> for LenraComponent {
    fn from(value: Textfield) -> Self {
        Self::Textfield(value)
    }
}
impl From<Toggle> for LenraComponent {
    fn from(value: Toggle) -> Self {
        Self::Toggle(value)
    }
}
impl From<View> for LenraComponent {
    fn from(value: View) -> Self {
        Self::View(value)
    }
}
impl From<Wrap> for LenraComponent {
    fn from(value: Wrap) -> Self {
        Self::Wrap(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Listener {
    ///The name of the listener to call
    pub name: ListenerName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<DefsProps>,
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Listener> for Listener {
    fn from(value: &Listener) -> Self {
        value.clone()
    }
}
impl Listener {
    pub fn builder() -> builder::Listener {
        builder::Listener::default()
    }
}
///The name of the listener to call
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ListenerName(String);
impl std::ops::Deref for ListenerName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ListenerName> for String {
    fn from(value: ListenerName) -> Self {
        value.0
    }
}
impl From<&ListenerName> for ListenerName {
    fn from(value: &ListenerName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ListenerName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^(@lenra:)?[a-zA-Z_$][a-zA-Z_.$0-9]*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(@lenra:)?[a-zA-Z_$][a-zA-Z_.$0-9]*$\"",
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ListenerName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListenerName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListenerName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
///Element of type Menu
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Menu {
    ///The menu items
    pub children: Vec<LenraComponent>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Menu> for Menu {
    fn from(value: &Menu) -> Self {
        value.clone()
    }
}
impl Menu {
    pub fn builder() -> builder::Menu {
        builder::Menu::default()
    }
}
///Element of type MenuItem
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MenuItem {
    ///Whether the element should be disabled or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    ///Whether the element is selected or not.
    #[serde(rename = "isSelected", default, skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,
    #[serde(rename = "onPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Listener>,
    ///The text of the element
    pub text: String,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&MenuItem> for MenuItem {
    fn from(value: &MenuItem) -> Self {
        value.clone()
    }
}
impl MenuItem {
    pub fn builder() -> builder::MenuItem {
        builder::MenuItem::default()
    }
}
///Element of type OverlayEntry
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OverlayEntry {
    pub child: Box<LenraComponent>,
    ///Whether this entry must be included in the tree even if there is a fully opaque entry above it.
    #[serde(rename = "maintainState", default, skip_serializing_if = "Option::is_none")]
    pub maintain_state: Option<bool>,
    ///Whether this entry occludes the entire overlay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<bool>,
    ///Whether this entry should be shown.
    #[serde(rename = "showOverlay", default, skip_serializing_if = "Option::is_none")]
    pub show_overlay: Option<bool>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&OverlayEntry> for OverlayEntry {
    fn from(value: &OverlayEntry) -> Self {
        value.clone()
    }
}
impl OverlayEntry {
    pub fn builder() -> builder::OverlayEntry {
        builder::OverlayEntry::default()
    }
}
///Element of type Radio
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Radio {
    ///Whether the radio will be selected initially.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autofocus: Option<bool>,
    ///The value group of the radio
    #[serde(rename = "groupValue")]
    pub group_value: String,
    ///Configures the minimum size of the tap target.
    #[serde(
        rename = "materialTapTargetSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub material_tap_target_size: Option<StylesMaterialTapTargetSize>,
    ///The name that will be used in the form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "onPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Listener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StylesRadioStyle>,
    ///Whether the radio is allowed to go from checked to unchecked when clicking on it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toggleable: Option<bool>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    ///The value of the radio
    pub value: String,
}
impl From<&Radio> for Radio {
    fn from(value: &Radio) -> Self {
        value.clone()
    }
}
impl Radio {
    pub fn builder() -> builder::Radio {
        builder::Radio::default()
    }
}
///Element of type Slider
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Slider {
    ///Whether the slider should be focused initially.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autofocus: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisions: Option<f64>,
    ///The label of the slider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    ///The name that will be used in the form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The callback to be invoked when the slider is released.
    #[serde(rename = "onChangeEnd", default, skip_serializing_if = "Option::is_none")]
    pub on_change_end: Option<Listener>,
    ///The callback to be invoked when the slider is pressed.
    #[serde(rename = "onChangeStart", default, skip_serializing_if = "Option::is_none")]
    pub on_change_start: Option<Listener>,
    ///The callback to be invoked when the slider value changes.
    #[serde(rename = "onChanged", default, skip_serializing_if = "Option::is_none")]
    pub on_changed: Option<Listener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StylesSliderStyle>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl From<&Slider> for Slider {
    fn from(value: &Slider) -> Self {
        value.clone()
    }
}
impl Slider {
    pub fn builder() -> builder::Slider {
        builder::Slider::default()
    }
}
///Element of type Stack
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<StylesAlignment>,
    ///The children of the Stack.
    pub children: Vec<LenraComponent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit: Option<StylesStackFit>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&Stack> for Stack {
    fn from(value: &Stack) -> Self {
        value.clone()
    }
}
impl Stack {
    pub fn builder() -> builder::Stack {
        builder::Stack::default()
    }
}
///Element of type StatusSticker
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusSticker {
    ///the status of the element
    pub status: StatusStickerStatus,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&StatusSticker> for StatusSticker {
    fn from(value: &StatusSticker) -> Self {
        value.clone()
    }
}
impl StatusSticker {
    pub fn builder() -> builder::StatusSticker {
        builder::StatusSticker::default()
    }
}
///the status of the element
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StatusStickerStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "pending")]
    Pending,
}
impl From<&StatusStickerStatus> for StatusStickerStatus {
    fn from(value: &StatusStickerStatus) -> Self {
        value.clone()
    }
}
impl ToString for StatusStickerStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "success".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Error => "error".to_string(),
            Self::Pending => "pending".to_string(),
        }
    }
}
impl std::str::FromStr for StatusStickerStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "success" => Ok(Self::Success),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "pending" => Ok(Self::Pending),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StatusStickerStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StatusStickerStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StatusStickerStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The alignment to use.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesAlignment {
    #[serde(rename = "bottomCenter")]
    BottomCenter,
    #[serde(rename = "bottomLeft")]
    BottomLeft,
    #[serde(rename = "bottomRight")]
    BottomRight,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "centerLeft")]
    CenterLeft,
    #[serde(rename = "centerRight")]
    CenterRight,
    #[serde(rename = "topCenter")]
    TopCenter,
    #[serde(rename = "topLeft")]
    TopLeft,
    #[serde(rename = "topRight")]
    TopRight,
}
impl From<&StylesAlignment> for StylesAlignment {
    fn from(value: &StylesAlignment) -> Self {
        value.clone()
    }
}
impl ToString for StylesAlignment {
    fn to_string(&self) -> String {
        match *self {
            Self::BottomCenter => "bottomCenter".to_string(),
            Self::BottomLeft => "bottomLeft".to_string(),
            Self::BottomRight => "bottomRight".to_string(),
            Self::Center => "center".to_string(),
            Self::CenterLeft => "centerLeft".to_string(),
            Self::CenterRight => "centerRight".to_string(),
            Self::TopCenter => "topCenter".to_string(),
            Self::TopLeft => "topLeft".to_string(),
            Self::TopRight => "topRight".to_string(),
        }
    }
}
impl std::str::FromStr for StylesAlignment {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "bottomCenter" => Ok(Self::BottomCenter),
            "bottomLeft" => Ok(Self::BottomLeft),
            "bottomRight" => Ok(Self::BottomRight),
            "center" => Ok(Self::Center),
            "centerLeft" => Ok(Self::CenterLeft),
            "centerRight" => Ok(Self::CenterRight),
            "topCenter" => Ok(Self::TopCenter),
            "topLeft" => Ok(Self::TopLeft),
            "topRight" => Ok(Self::TopRight),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesAlignment {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesAlignment {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesAlignment {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The AutofillHints enum to handle textfield's autocompletion.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StylesAutofillHints(pub Vec<StylesAutofillHintsItem>);
impl std::ops::Deref for StylesAutofillHints {
    type Target = Vec<StylesAutofillHintsItem>;
    fn deref(&self) -> &Vec<StylesAutofillHintsItem> {
        &self.0
    }
}
impl From<StylesAutofillHints> for Vec<StylesAutofillHintsItem> {
    fn from(value: StylesAutofillHints) -> Self {
        value.0
    }
}
impl From<&StylesAutofillHints> for StylesAutofillHints {
    fn from(value: &StylesAutofillHints) -> Self {
        value.clone()
    }
}
impl From<Vec<StylesAutofillHintsItem>> for StylesAutofillHints {
    fn from(value: Vec<StylesAutofillHintsItem>) -> Self {
        Self(value)
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesAutofillHintsItem {
    #[serde(rename = "addressCity")]
    AddressCity,
    #[serde(rename = "addressCityAndState")]
    AddressCityAndState,
    #[serde(rename = "addressState")]
    AddressState,
    #[serde(rename = "birthday")]
    Birthday,
    #[serde(rename = "birthdayDay")]
    BirthdayDay,
    #[serde(rename = "birthdayMonth")]
    BirthdayMonth,
    #[serde(rename = "birthdayYear")]
    BirthdayYear,
    #[serde(rename = "countryCode")]
    CountryCode,
    #[serde(rename = "countryName")]
    CountryName,
    #[serde(rename = "creditCardExpirationDate")]
    CreditCardExpirationDate,
    #[serde(rename = "creditCardExpirationDay")]
    CreditCardExpirationDay,
    #[serde(rename = "creditCardExpirationMonth")]
    CreditCardExpirationMonth,
    #[serde(rename = "creditCardExpirationYear")]
    CreditCardExpirationYear,
    #[serde(rename = "creditCardFamilyName")]
    CreditCardFamilyName,
    #[serde(rename = "creditCardGivenName")]
    CreditCardGivenName,
    #[serde(rename = "creditCardMiddleName")]
    CreditCardMiddleName,
    #[serde(rename = "creditCardName")]
    CreditCardName,
    #[serde(rename = "creditCardNumber")]
    CreditCardNumber,
    #[serde(rename = "creditCardSecurityCode")]
    CreditCardSecurityCode,
    #[serde(rename = "creditCardType")]
    CreditCardType,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "familyName")]
    FamilyName,
    #[serde(rename = "fullStreetAddress")]
    FullStreetAddress,
    #[serde(rename = "gender")]
    Gender,
    #[serde(rename = "givenName")]
    GivenName,
    #[serde(rename = "impp")]
    Impp,
    #[serde(rename = "jobTitle")]
    JobTitle,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "middleInitial")]
    MiddleInitial,
    #[serde(rename = "middleName")]
    MiddleName,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "namePrefix")]
    NamePrefix,
    #[serde(rename = "nameSuffix")]
    NameSuffix,
    #[serde(rename = "newPassword")]
    NewPassword,
    #[serde(rename = "newUsername")]
    NewUsername,
    #[serde(rename = "nickname")]
    Nickname,
    #[serde(rename = "oneTimeCode")]
    OneTimeCode,
    #[serde(rename = "organizationName")]
    OrganizationName,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "postalAddress")]
    PostalAddress,
    #[serde(rename = "postalAddressExtended")]
    PostalAddressExtended,
    #[serde(rename = "postalAddressExtendedPostalCode")]
    PostalAddressExtendedPostalCode,
    #[serde(rename = "postalCode")]
    PostalCode,
    #[serde(rename = "streetAddressLevel1")]
    StreetAddressLevel1,
    #[serde(rename = "streetAddressLevel2")]
    StreetAddressLevel2,
    #[serde(rename = "streetAddressLevel3")]
    StreetAddressLevel3,
    #[serde(rename = "streetAddressLevel4")]
    StreetAddressLevel4,
    #[serde(rename = "streetAddressLine1")]
    StreetAddressLine1,
    #[serde(rename = "streetAddressLine2")]
    StreetAddressLine2,
    #[serde(rename = "streetAddressLine3")]
    StreetAddressLine3,
    #[serde(rename = "sublocality")]
    Sublocality,
    #[serde(rename = "telephoneNumber")]
    TelephoneNumber,
    #[serde(rename = "telephoneNumberAreaCode")]
    TelephoneNumberAreaCode,
    #[serde(rename = "telephoneNumberCountryCode")]
    TelephoneNumberCountryCode,
    #[serde(rename = "telephoneNumberDevice")]
    TelephoneNumberDevice,
    #[serde(rename = "telephoneNumberExtension")]
    TelephoneNumberExtension,
    #[serde(rename = "telephoneNumberLocal")]
    TelephoneNumberLocal,
    #[serde(rename = "telephoneNumberLocalPrefix")]
    TelephoneNumberLocalPrefix,
    #[serde(rename = "telephoneNumberLocalSuffix")]
    TelephoneNumberLocalSuffix,
    #[serde(rename = "telephoneNumberNational")]
    TelephoneNumberNational,
    #[serde(rename = "transactionAmount")]
    TransactionAmount,
    #[serde(rename = "transactionCurrency")]
    TransactionCurrency,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "username")]
    Username,
}
impl From<&StylesAutofillHintsItem> for StylesAutofillHintsItem {
    fn from(value: &StylesAutofillHintsItem) -> Self {
        value.clone()
    }
}
impl ToString for StylesAutofillHintsItem {
    fn to_string(&self) -> String {
        match *self {
            Self::AddressCity => "addressCity".to_string(),
            Self::AddressCityAndState => "addressCityAndState".to_string(),
            Self::AddressState => "addressState".to_string(),
            Self::Birthday => "birthday".to_string(),
            Self::BirthdayDay => "birthdayDay".to_string(),
            Self::BirthdayMonth => "birthdayMonth".to_string(),
            Self::BirthdayYear => "birthdayYear".to_string(),
            Self::CountryCode => "countryCode".to_string(),
            Self::CountryName => "countryName".to_string(),
            Self::CreditCardExpirationDate => "creditCardExpirationDate".to_string(),
            Self::CreditCardExpirationDay => "creditCardExpirationDay".to_string(),
            Self::CreditCardExpirationMonth => "creditCardExpirationMonth".to_string(),
            Self::CreditCardExpirationYear => "creditCardExpirationYear".to_string(),
            Self::CreditCardFamilyName => "creditCardFamilyName".to_string(),
            Self::CreditCardGivenName => "creditCardGivenName".to_string(),
            Self::CreditCardMiddleName => "creditCardMiddleName".to_string(),
            Self::CreditCardName => "creditCardName".to_string(),
            Self::CreditCardNumber => "creditCardNumber".to_string(),
            Self::CreditCardSecurityCode => "creditCardSecurityCode".to_string(),
            Self::CreditCardType => "creditCardType".to_string(),
            Self::Email => "email".to_string(),
            Self::FamilyName => "familyName".to_string(),
            Self::FullStreetAddress => "fullStreetAddress".to_string(),
            Self::Gender => "gender".to_string(),
            Self::GivenName => "givenName".to_string(),
            Self::Impp => "impp".to_string(),
            Self::JobTitle => "jobTitle".to_string(),
            Self::Language => "language".to_string(),
            Self::Location => "location".to_string(),
            Self::MiddleInitial => "middleInitial".to_string(),
            Self::MiddleName => "middleName".to_string(),
            Self::Name => "name".to_string(),
            Self::NamePrefix => "namePrefix".to_string(),
            Self::NameSuffix => "nameSuffix".to_string(),
            Self::NewPassword => "newPassword".to_string(),
            Self::NewUsername => "newUsername".to_string(),
            Self::Nickname => "nickname".to_string(),
            Self::OneTimeCode => "oneTimeCode".to_string(),
            Self::OrganizationName => "organizationName".to_string(),
            Self::Password => "password".to_string(),
            Self::Photo => "photo".to_string(),
            Self::PostalAddress => "postalAddress".to_string(),
            Self::PostalAddressExtended => "postalAddressExtended".to_string(),
            Self::PostalAddressExtendedPostalCode => {
                "postalAddressExtendedPostalCode".to_string()
            }
            Self::PostalCode => "postalCode".to_string(),
            Self::StreetAddressLevel1 => "streetAddressLevel1".to_string(),
            Self::StreetAddressLevel2 => "streetAddressLevel2".to_string(),
            Self::StreetAddressLevel3 => "streetAddressLevel3".to_string(),
            Self::StreetAddressLevel4 => "streetAddressLevel4".to_string(),
            Self::StreetAddressLine1 => "streetAddressLine1".to_string(),
            Self::StreetAddressLine2 => "streetAddressLine2".to_string(),
            Self::StreetAddressLine3 => "streetAddressLine3".to_string(),
            Self::Sublocality => "sublocality".to_string(),
            Self::TelephoneNumber => "telephoneNumber".to_string(),
            Self::TelephoneNumberAreaCode => "telephoneNumberAreaCode".to_string(),
            Self::TelephoneNumberCountryCode => "telephoneNumberCountryCode".to_string(),
            Self::TelephoneNumberDevice => "telephoneNumberDevice".to_string(),
            Self::TelephoneNumberExtension => "telephoneNumberExtension".to_string(),
            Self::TelephoneNumberLocal => "telephoneNumberLocal".to_string(),
            Self::TelephoneNumberLocalPrefix => "telephoneNumberLocalPrefix".to_string(),
            Self::TelephoneNumberLocalSuffix => "telephoneNumberLocalSuffix".to_string(),
            Self::TelephoneNumberNational => "telephoneNumberNational".to_string(),
            Self::TransactionAmount => "transactionAmount".to_string(),
            Self::TransactionCurrency => "transactionCurrency".to_string(),
            Self::Url => "url".to_string(),
            Self::Username => "username".to_string(),
        }
    }
}
impl std::str::FromStr for StylesAutofillHintsItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "addressCity" => Ok(Self::AddressCity),
            "addressCityAndState" => Ok(Self::AddressCityAndState),
            "addressState" => Ok(Self::AddressState),
            "birthday" => Ok(Self::Birthday),
            "birthdayDay" => Ok(Self::BirthdayDay),
            "birthdayMonth" => Ok(Self::BirthdayMonth),
            "birthdayYear" => Ok(Self::BirthdayYear),
            "countryCode" => Ok(Self::CountryCode),
            "countryName" => Ok(Self::CountryName),
            "creditCardExpirationDate" => Ok(Self::CreditCardExpirationDate),
            "creditCardExpirationDay" => Ok(Self::CreditCardExpirationDay),
            "creditCardExpirationMonth" => Ok(Self::CreditCardExpirationMonth),
            "creditCardExpirationYear" => Ok(Self::CreditCardExpirationYear),
            "creditCardFamilyName" => Ok(Self::CreditCardFamilyName),
            "creditCardGivenName" => Ok(Self::CreditCardGivenName),
            "creditCardMiddleName" => Ok(Self::CreditCardMiddleName),
            "creditCardName" => Ok(Self::CreditCardName),
            "creditCardNumber" => Ok(Self::CreditCardNumber),
            "creditCardSecurityCode" => Ok(Self::CreditCardSecurityCode),
            "creditCardType" => Ok(Self::CreditCardType),
            "email" => Ok(Self::Email),
            "familyName" => Ok(Self::FamilyName),
            "fullStreetAddress" => Ok(Self::FullStreetAddress),
            "gender" => Ok(Self::Gender),
            "givenName" => Ok(Self::GivenName),
            "impp" => Ok(Self::Impp),
            "jobTitle" => Ok(Self::JobTitle),
            "language" => Ok(Self::Language),
            "location" => Ok(Self::Location),
            "middleInitial" => Ok(Self::MiddleInitial),
            "middleName" => Ok(Self::MiddleName),
            "name" => Ok(Self::Name),
            "namePrefix" => Ok(Self::NamePrefix),
            "nameSuffix" => Ok(Self::NameSuffix),
            "newPassword" => Ok(Self::NewPassword),
            "newUsername" => Ok(Self::NewUsername),
            "nickname" => Ok(Self::Nickname),
            "oneTimeCode" => Ok(Self::OneTimeCode),
            "organizationName" => Ok(Self::OrganizationName),
            "password" => Ok(Self::Password),
            "photo" => Ok(Self::Photo),
            "postalAddress" => Ok(Self::PostalAddress),
            "postalAddressExtended" => Ok(Self::PostalAddressExtended),
            "postalAddressExtendedPostalCode" => {
                Ok(Self::PostalAddressExtendedPostalCode)
            }
            "postalCode" => Ok(Self::PostalCode),
            "streetAddressLevel1" => Ok(Self::StreetAddressLevel1),
            "streetAddressLevel2" => Ok(Self::StreetAddressLevel2),
            "streetAddressLevel3" => Ok(Self::StreetAddressLevel3),
            "streetAddressLevel4" => Ok(Self::StreetAddressLevel4),
            "streetAddressLine1" => Ok(Self::StreetAddressLine1),
            "streetAddressLine2" => Ok(Self::StreetAddressLine2),
            "streetAddressLine3" => Ok(Self::StreetAddressLine3),
            "sublocality" => Ok(Self::Sublocality),
            "telephoneNumber" => Ok(Self::TelephoneNumber),
            "telephoneNumberAreaCode" => Ok(Self::TelephoneNumberAreaCode),
            "telephoneNumberCountryCode" => Ok(Self::TelephoneNumberCountryCode),
            "telephoneNumberDevice" => Ok(Self::TelephoneNumberDevice),
            "telephoneNumberExtension" => Ok(Self::TelephoneNumberExtension),
            "telephoneNumberLocal" => Ok(Self::TelephoneNumberLocal),
            "telephoneNumberLocalPrefix" => Ok(Self::TelephoneNumberLocalPrefix),
            "telephoneNumberLocalSuffix" => Ok(Self::TelephoneNumberLocalSuffix),
            "telephoneNumberNational" => Ok(Self::TelephoneNumberNational),
            "transactionAmount" => Ok(Self::TransactionAmount),
            "transactionCurrency" => Ok(Self::TransactionCurrency),
            "url" => Ok(Self::Url),
            "username" => Ok(Self::Username),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesAutofillHintsItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesAutofillHintsItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesAutofillHintsItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type Border
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBorder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom: Option<StylesBorderSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<StylesBorderSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<StylesBorderSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<StylesBorderSide>,
}
impl From<&StylesBorder> for StylesBorder {
    fn from(value: &StylesBorder) -> Self {
        value.clone()
    }
}
impl StylesBorder {
    pub fn builder() -> builder::StylesBorder {
        builder::StylesBorder::default()
    }
}
///Element of type BorderRadius
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBorderRadius {
    #[serde(rename = "bottomLeft", default, skip_serializing_if = "Option::is_none")]
    pub bottom_left: Option<StylesBorderRadiusDefinitionsRadiusType>,
    #[serde(rename = "bottomRight", default, skip_serializing_if = "Option::is_none")]
    pub bottom_right: Option<StylesBorderRadiusDefinitionsRadiusType>,
    #[serde(rename = "topLeft", default, skip_serializing_if = "Option::is_none")]
    pub top_left: Option<StylesBorderRadiusDefinitionsRadiusType>,
    #[serde(rename = "topRight", default, skip_serializing_if = "Option::is_none")]
    pub top_right: Option<StylesBorderRadiusDefinitionsRadiusType>,
}
impl From<&StylesBorderRadius> for StylesBorderRadius {
    fn from(value: &StylesBorderRadius) -> Self {
        value.clone()
    }
}
impl StylesBorderRadius {
    pub fn builder() -> builder::StylesBorderRadius {
        builder::StylesBorderRadius::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBorderRadiusDefinitionsRadiusType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}
impl From<&StylesBorderRadiusDefinitionsRadiusType>
for StylesBorderRadiusDefinitionsRadiusType {
    fn from(value: &StylesBorderRadiusDefinitionsRadiusType) -> Self {
        value.clone()
    }
}
impl StylesBorderRadiusDefinitionsRadiusType {
    pub fn builder() -> builder::StylesBorderRadiusDefinitionsRadiusType {
        builder::StylesBorderRadiusDefinitionsRadiusType::default()
    }
}
///Element of type BorderSide
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBorderSide {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<StylesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}
impl From<&StylesBorderSide> for StylesBorderSide {
    fn from(value: &StylesBorderSide) -> Self {
        value.clone()
    }
}
impl StylesBorderSide {
    pub fn builder() -> builder::StylesBorderSide {
        builder::StylesBorderSide::default()
    }
}
///Element of type BoxConstraints
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBoxConstraints {
    #[serde(rename = "maxHeight", default, skip_serializing_if = "Option::is_none")]
    pub max_height: Option<f64>,
    #[serde(rename = "maxWidth", default, skip_serializing_if = "Option::is_none")]
    pub max_width: Option<f64>,
    #[serde(rename = "minHeight", default, skip_serializing_if = "Option::is_none")]
    pub min_height: Option<f64>,
    #[serde(rename = "minWidth", default, skip_serializing_if = "Option::is_none")]
    pub min_width: Option<f64>,
}
impl From<&StylesBoxConstraints> for StylesBoxConstraints {
    fn from(value: &StylesBoxConstraints) -> Self {
        value.clone()
    }
}
impl StylesBoxConstraints {
    pub fn builder() -> builder::StylesBoxConstraints {
        builder::StylesBoxConstraints::default()
    }
}
///Element of type BoxDecoration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBoxDecoration {
    #[serde(rename = "borderRadius", default, skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<StylesBorderRadius>,
    #[serde(rename = "boxShadow", default, skip_serializing_if = "Option::is_none")]
    pub box_shadow: Option<StylesBoxShadow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<StylesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<StylesBoxShape>,
}
impl From<&StylesBoxDecoration> for StylesBoxDecoration {
    fn from(value: &StylesBoxDecoration) -> Self {
        value.clone()
    }
}
impl StylesBoxDecoration {
    pub fn builder() -> builder::StylesBoxDecoration {
        builder::StylesBoxDecoration::default()
    }
}
///How the box should be fitted in its parent.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesBoxFit {
    #[serde(rename = "contain")]
    Contain,
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "fitHeight")]
    FitHeight,
    #[serde(rename = "fitWidth")]
    FitWidth,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "scaleDown")]
    ScaleDown,
}
impl From<&StylesBoxFit> for StylesBoxFit {
    fn from(value: &StylesBoxFit) -> Self {
        value.clone()
    }
}
impl ToString for StylesBoxFit {
    fn to_string(&self) -> String {
        match *self {
            Self::Contain => "contain".to_string(),
            Self::Cover => "cover".to_string(),
            Self::Fill => "fill".to_string(),
            Self::FitHeight => "fitHeight".to_string(),
            Self::FitWidth => "fitWidth".to_string(),
            Self::None => "none".to_string(),
            Self::ScaleDown => "scaleDown".to_string(),
        }
    }
}
impl std::str::FromStr for StylesBoxFit {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "contain" => Ok(Self::Contain),
            "cover" => Ok(Self::Cover),
            "fill" => Ok(Self::Fill),
            "fitHeight" => Ok(Self::FitHeight),
            "fitWidth" => Ok(Self::FitWidth),
            "none" => Ok(Self::None),
            "scaleDown" => Ok(Self::ScaleDown),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesBoxFit {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesBoxFit {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesBoxFit {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type BoxHeightStyle.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesBoxHeightStyle {
    #[serde(rename = "includeLineSpacingBottom")]
    IncludeLineSpacingBottom,
    #[serde(rename = "includeLineSpacingMiddle")]
    IncludeLineSpacingMiddle,
    #[serde(rename = "includeLineSpacingTop")]
    IncludeLineSpacingTop,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "strut")]
    Strut,
    #[serde(rename = "tight")]
    Tight,
}
impl From<&StylesBoxHeightStyle> for StylesBoxHeightStyle {
    fn from(value: &StylesBoxHeightStyle) -> Self {
        value.clone()
    }
}
impl ToString for StylesBoxHeightStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::IncludeLineSpacingBottom => "includeLineSpacingBottom".to_string(),
            Self::IncludeLineSpacingMiddle => "includeLineSpacingMiddle".to_string(),
            Self::IncludeLineSpacingTop => "includeLineSpacingTop".to_string(),
            Self::Max => "max".to_string(),
            Self::Strut => "strut".to_string(),
            Self::Tight => "tight".to_string(),
        }
    }
}
impl std::str::FromStr for StylesBoxHeightStyle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "includeLineSpacingBottom" => Ok(Self::IncludeLineSpacingBottom),
            "includeLineSpacingMiddle" => Ok(Self::IncludeLineSpacingMiddle),
            "includeLineSpacingTop" => Ok(Self::IncludeLineSpacingTop),
            "max" => Ok(Self::Max),
            "strut" => Ok(Self::Strut),
            "tight" => Ok(Self::Tight),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesBoxHeightStyle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesBoxHeightStyle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesBoxHeightStyle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type BoxShadow
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesBoxShadow {
    #[serde(rename = "blurRadius", default, skip_serializing_if = "Option::is_none")]
    pub blur_radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<StylesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<StylesOffset>,
    #[serde(rename = "spreadRadius", default, skip_serializing_if = "Option::is_none")]
    pub spread_radius: Option<f64>,
}
impl From<&StylesBoxShadow> for StylesBoxShadow {
    fn from(value: &StylesBoxShadow) -> Self {
        value.clone()
    }
}
impl StylesBoxShadow {
    pub fn builder() -> builder::StylesBoxShadow {
        builder::StylesBoxShadow::default()
    }
}
///The BoxShape enum, used to define the shape of a box.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesBoxShape {
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "rectangle")]
    Rectangle,
}
impl From<&StylesBoxShape> for StylesBoxShape {
    fn from(value: &StylesBoxShape) -> Self {
        value.clone()
    }
}
impl ToString for StylesBoxShape {
    fn to_string(&self) -> String {
        match *self {
            Self::Circle => "circle".to_string(),
            Self::Rectangle => "rectangle".to_string(),
        }
    }
}
impl std::str::FromStr for StylesBoxShape {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "circle" => Ok(Self::Circle),
            "rectangle" => Ok(Self::Rectangle),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesBoxShape {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesBoxShape {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesBoxShape {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type BoxWidthStyle.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesBoxWidthStyle {
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "tight")]
    Tight,
}
impl From<&StylesBoxWidthStyle> for StylesBoxWidthStyle {
    fn from(value: &StylesBoxWidthStyle) -> Self {
        value.clone()
    }
}
impl ToString for StylesBoxWidthStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::Max => "max".to_string(),
            Self::Tight => "tight".to_string(),
        }
    }
}
impl std::str::FromStr for StylesBoxWidthStyle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "max" => Ok(Self::Max),
            "tight" => Ok(Self::Tight),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesBoxWidthStyle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesBoxWidthStyle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesBoxWidthStyle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type Brightness.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesBrightness {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}
impl From<&StylesBrightness> for StylesBrightness {
    fn from(value: &StylesBrightness) -> Self {
        value.clone()
    }
}
impl ToString for StylesBrightness {
    fn to_string(&self) -> String {
        match *self {
            Self::Dark => "dark".to_string(),
            Self::Light => "light".to_string(),
        }
    }
}
impl std::str::FromStr for StylesBrightness {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesBrightness {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesBrightness {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesBrightness {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type CarouselOptions
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesCarouselOptions {
    #[serde(rename = "aspectRatio", default, skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "autoPlay", default, skip_serializing_if = "Option::is_none")]
    pub auto_play: Option<bool>,
    #[serde(
        rename = "autoPlayAnimationDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_play_animation_duration: Option<StylesDuration>,
    #[serde(
        rename = "autoPlayInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_play_interval: Option<StylesDuration>,
    #[serde(
        rename = "enableInfiniteScroll",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_infinite_scroll: Option<bool>,
    #[serde(
        rename = "enlargeCenterPage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enlarge_center_page: Option<bool>,
    #[serde(
        rename = "enlargeStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enlarge_strategy: Option<StylesCarouselOptionsEnlargeStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(rename = "initialPage", default, skip_serializing_if = "Option::is_none")]
    pub initial_page: Option<i64>,
    #[serde(
        rename = "pauseAutoPlayOnTouch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_auto_play_on_touch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(
        rename = "scrollDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_direction: Option<StylesDirection>,
    #[serde(
        rename = "viewportFraction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub viewport_fraction: Option<f64>,
}
impl From<&StylesCarouselOptions> for StylesCarouselOptions {
    fn from(value: &StylesCarouselOptions) -> Self {
        value.clone()
    }
}
impl StylesCarouselOptions {
    pub fn builder() -> builder::StylesCarouselOptions {
        builder::StylesCarouselOptions::default()
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesCarouselOptionsEnlargeStrategy {
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "zoom")]
    Zoom,
}
impl From<&StylesCarouselOptionsEnlargeStrategy>
for StylesCarouselOptionsEnlargeStrategy {
    fn from(value: &StylesCarouselOptionsEnlargeStrategy) -> Self {
        value.clone()
    }
}
impl ToString for StylesCarouselOptionsEnlargeStrategy {
    fn to_string(&self) -> String {
        match *self {
            Self::Scale => "scale".to_string(),
            Self::Height => "height".to_string(),
            Self::Zoom => "zoom".to_string(),
        }
    }
}
impl std::str::FromStr for StylesCarouselOptionsEnlargeStrategy {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "scale" => Ok(Self::Scale),
            "height" => Ok(Self::Height),
            "zoom" => Ok(Self::Zoom),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesCarouselOptionsEnlargeStrategy {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesCarouselOptionsEnlargeStrategy {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesCarouselOptionsEnlargeStrategy {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type CheckboxStyle
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesCheckboxStyle {
    #[serde(rename = "activeColor", default, skip_serializing_if = "Option::is_none")]
    pub active_color: Option<StylesColor>,
    #[serde(rename = "checkColor", default, skip_serializing_if = "Option::is_none")]
    pub check_color: Option<StylesColor>,
    #[serde(rename = "focusColor", default, skip_serializing_if = "Option::is_none")]
    pub focus_color: Option<StylesColor>,
    #[serde(rename = "hoverColor", default, skip_serializing_if = "Option::is_none")]
    pub hover_color: Option<StylesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<StylesOutlinedBorder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<StylesBorderSide>,
    #[serde(rename = "splashRadius", default, skip_serializing_if = "Option::is_none")]
    pub splash_radius: Option<f64>,
    #[serde(rename = "visualDensity", default, skip_serializing_if = "Option::is_none")]
    pub visual_density: Option<StylesVisualDensity>,
}
impl From<&StylesCheckboxStyle> for StylesCheckboxStyle {
    fn from(value: &StylesCheckboxStyle) -> Self {
        value.clone()
    }
}
impl StylesCheckboxStyle {
    pub fn builder() -> builder::StylesCheckboxStyle {
        builder::StylesCheckboxStyle::default()
    }
}
///Color type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StylesColor(pub i64);
impl std::ops::Deref for StylesColor {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl From<StylesColor> for i64 {
    fn from(value: StylesColor) -> Self {
        value.0
    }
}
impl From<&StylesColor> for StylesColor {
    fn from(value: &StylesColor) -> Self {
        value.clone()
    }
}
impl From<i64> for StylesColor {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for StylesColor {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for StylesColor {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesColor {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesColor {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for StylesColor {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
///The direction of the component (horizontal/vertical)
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesDirection {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}
impl From<&StylesDirection> for StylesDirection {
    fn from(value: &StylesDirection) -> Self {
        value.clone()
    }
}
impl ToString for StylesDirection {
    fn to_string(&self) -> String {
        match *self {
            Self::Horizontal => "horizontal".to_string(),
            Self::Vertical => "vertical".to_string(),
        }
    }
}
impl std::str::FromStr for StylesDirection {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "horizontal" => Ok(Self::Horizontal),
            "vertical" => Ok(Self::Vertical),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesDirection {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesDirection {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesDirection {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type DragStartBehavior.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesDragStartBehavior {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "down")]
    Down,
}
impl From<&StylesDragStartBehavior> for StylesDragStartBehavior {
    fn from(value: &StylesDragStartBehavior) -> Self {
        value.clone()
    }
}
impl ToString for StylesDragStartBehavior {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::Down => "down".to_string(),
        }
    }
}
impl std::str::FromStr for StylesDragStartBehavior {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "down" => Ok(Self::Down),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesDragStartBehavior {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesDragStartBehavior {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesDragStartBehavior {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type Duration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesDuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub microseconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milliseconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}
impl From<&StylesDuration> for StylesDuration {
    fn from(value: &StylesDuration) -> Self {
        value.clone()
    }
}
impl StylesDuration {
    pub fn builder() -> builder::StylesDuration {
        builder::StylesDuration::default()
    }
}
///The filter quality to use.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesFilterQuality {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "none")]
    None,
}
impl From<&StylesFilterQuality> for StylesFilterQuality {
    fn from(value: &StylesFilterQuality) -> Self {
        value.clone()
    }
}
impl ToString for StylesFilterQuality {
    fn to_string(&self) -> String {
        match *self {
            Self::High => "high".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Low => "low".to_string(),
            Self::None => "none".to_string(),
        }
    }
}
impl std::str::FromStr for StylesFilterQuality {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            "none" => Ok(Self::None),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesFilterQuality {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesFilterQuality {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesFilterQuality {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///How a flexible child is inscribed into the available space.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesFlexFit {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "tight")]
    Tight,
}
impl From<&StylesFlexFit> for StylesFlexFit {
    fn from(value: &StylesFlexFit) -> Self {
        value.clone()
    }
}
impl ToString for StylesFlexFit {
    fn to_string(&self) -> String {
        match *self {
            Self::Loose => "loose".to_string(),
            Self::Tight => "tight".to_string(),
        }
    }
}
impl std::str::FromStr for StylesFlexFit {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "loose" => Ok(Self::Loose),
            "tight" => Ok(Self::Tight),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesFlexFit {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesFlexFit {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesFlexFit {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type FloatingLabelBehavior.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesFloatingLabelBehavior {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "never")]
    Never,
}
impl From<&StylesFloatingLabelBehavior> for StylesFloatingLabelBehavior {
    fn from(value: &StylesFloatingLabelBehavior) -> Self {
        value.clone()
    }
}
impl ToString for StylesFloatingLabelBehavior {
    fn to_string(&self) -> String {
        match *self {
            Self::Always => "always".to_string(),
            Self::Auto => "auto".to_string(),
            Self::Never => "never".to_string(),
        }
    }
}
impl std::str::FromStr for StylesFloatingLabelBehavior {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "always" => Ok(Self::Always),
            "auto" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesFloatingLabelBehavior {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesFloatingLabelBehavior {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesFloatingLabelBehavior {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///All of the possible values for an Icon.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesIconName {
    #[serde(rename = "ac_unit")]
    AcUnit,
    #[serde(rename = "access_alarm")]
    AccessAlarm,
    #[serde(rename = "access_alarms")]
    AccessAlarms,
    #[serde(rename = "access_time")]
    AccessTime,
    #[serde(rename = "access_time_filled")]
    AccessTimeFilled,
    #[serde(rename = "accessibility")]
    Accessibility,
    #[serde(rename = "accessibility_new")]
    AccessibilityNew,
    #[serde(rename = "accessible")]
    Accessible,
    #[serde(rename = "accessible_forward")]
    AccessibleForward,
    #[serde(rename = "account_balance")]
    AccountBalance,
    #[serde(rename = "account_balance_wallet")]
    AccountBalanceWallet,
    #[serde(rename = "account_box")]
    AccountBox,
    #[serde(rename = "account_circle")]
    AccountCircle,
    #[serde(rename = "account_tree")]
    AccountTree,
    #[serde(rename = "ad_units")]
    AdUnits,
    #[serde(rename = "adb")]
    Adb,
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "add_a_photo")]
    AddAPhoto,
    #[serde(rename = "add_alarm")]
    AddAlarm,
    #[serde(rename = "add_alert")]
    AddAlert,
    #[serde(rename = "add_box")]
    AddBox,
    #[serde(rename = "add_business")]
    AddBusiness,
    #[serde(rename = "add_call")]
    AddCall,
    #[serde(rename = "add_chart")]
    AddChart,
    #[serde(rename = "add_circle")]
    AddCircle,
    #[serde(rename = "add_circle_outline")]
    AddCircleOutline,
    #[serde(rename = "add_comment")]
    AddComment,
    #[serde(rename = "add_ic_call")]
    AddIcCall,
    #[serde(rename = "add_link")]
    AddLink,
    #[serde(rename = "add_location")]
    AddLocation,
    #[serde(rename = "add_location_alt")]
    AddLocationAlt,
    #[serde(rename = "add_moderator")]
    AddModerator,
    #[serde(rename = "add_photo_alternate")]
    AddPhotoAlternate,
    #[serde(rename = "add_reaction")]
    AddReaction,
    #[serde(rename = "add_road")]
    AddRoad,
    #[serde(rename = "add_shopping_cart")]
    AddShoppingCart,
    #[serde(rename = "add_task")]
    AddTask,
    #[serde(rename = "add_to_drive")]
    AddToDrive,
    #[serde(rename = "add_to_home_screen")]
    AddToHomeScreen,
    #[serde(rename = "add_to_photos")]
    AddToPhotos,
    #[serde(rename = "add_to_queue")]
    AddToQueue,
    #[serde(rename = "addchart")]
    Addchart,
    #[serde(rename = "adjust")]
    Adjust,
    #[serde(rename = "admin_panel_settings")]
    AdminPanelSettings,
    #[serde(rename = "agriculture")]
    Agriculture,
    #[serde(rename = "air")]
    Air,
    #[serde(rename = "airline_seat_flat")]
    AirlineSeatFlat,
    #[serde(rename = "airline_seat_flat_angled")]
    AirlineSeatFlatAngled,
    #[serde(rename = "airline_seat_individual_suite")]
    AirlineSeatIndividualSuite,
    #[serde(rename = "airline_seat_legroom_extra")]
    AirlineSeatLegroomExtra,
    #[serde(rename = "airline_seat_legroom_normal")]
    AirlineSeatLegroomNormal,
    #[serde(rename = "airline_seat_legroom_reduced")]
    AirlineSeatLegroomReduced,
    #[serde(rename = "airline_seat_recline_extra")]
    AirlineSeatReclineExtra,
    #[serde(rename = "airline_seat_recline_normal")]
    AirlineSeatReclineNormal,
    #[serde(rename = "airplane_ticket")]
    AirplaneTicket,
    #[serde(rename = "airplanemode_active")]
    AirplanemodeActive,
    #[serde(rename = "airplanemode_inactive")]
    AirplanemodeInactive,
    #[serde(rename = "airplanemode_off")]
    AirplanemodeOff,
    #[serde(rename = "airplanemode_on")]
    AirplanemodeOn,
    #[serde(rename = "airplay")]
    Airplay,
    #[serde(rename = "airport_shuttle")]
    AirportShuttle,
    #[serde(rename = "alarm")]
    Alarm,
    #[serde(rename = "alarm_add")]
    AlarmAdd,
    #[serde(rename = "alarm_off")]
    AlarmOff,
    #[serde(rename = "alarm_on")]
    AlarmOn,
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "align_horizontal_center")]
    AlignHorizontalCenter,
    #[serde(rename = "align_horizontal_left")]
    AlignHorizontalLeft,
    #[serde(rename = "align_horizontal_right")]
    AlignHorizontalRight,
    #[serde(rename = "align_vertical_bottom")]
    AlignVerticalBottom,
    #[serde(rename = "align_vertical_center")]
    AlignVerticalCenter,
    #[serde(rename = "align_vertical_top")]
    AlignVerticalTop,
    #[serde(rename = "all_inbox")]
    AllInbox,
    #[serde(rename = "all_inclusive")]
    AllInclusive,
    #[serde(rename = "all_out")]
    AllOut,
    #[serde(rename = "alt_route")]
    AltRoute,
    #[serde(rename = "alternate_email")]
    AlternateEmail,
    #[serde(rename = "amp_stories")]
    AmpStories,
    #[serde(rename = "analytics")]
    Analytics,
    #[serde(rename = "anchor")]
    Anchor,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "animation")]
    Animation,
    #[serde(rename = "announcement")]
    Announcement,
    #[serde(rename = "aod")]
    Aod,
    #[serde(rename = "apartment")]
    Apartment,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "app_blocking")]
    AppBlocking,
    #[serde(rename = "app_registration")]
    AppRegistration,
    #[serde(rename = "app_settings_alt")]
    AppSettingsAlt,
    #[serde(rename = "approval")]
    Approval,
    #[serde(rename = "apps")]
    Apps,
    #[serde(rename = "architecture")]
    Architecture,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "arrow_back")]
    ArrowBack,
    #[serde(rename = "arrow_back_ios")]
    ArrowBackIos,
    #[serde(rename = "arrow_back_ios_new")]
    ArrowBackIosNew,
    #[serde(rename = "arrow_circle_down")]
    ArrowCircleDown,
    #[serde(rename = "arrow_circle_up")]
    ArrowCircleUp,
    #[serde(rename = "arrow_downward")]
    ArrowDownward,
    #[serde(rename = "arrow_drop_down")]
    ArrowDropDown,
    #[serde(rename = "arrow_drop_down_circle")]
    ArrowDropDownCircle,
    #[serde(rename = "arrow_drop_up")]
    ArrowDropUp,
    #[serde(rename = "arrow_forward")]
    ArrowForward,
    #[serde(rename = "arrow_forward_ios")]
    ArrowForwardIos,
    #[serde(rename = "arrow_left")]
    ArrowLeft,
    #[serde(rename = "arrow_right")]
    ArrowRight,
    #[serde(rename = "arrow_right_alt")]
    ArrowRightAlt,
    #[serde(rename = "arrow_upward")]
    ArrowUpward,
    #[serde(rename = "art_track")]
    ArtTrack,
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "aspect_ratio")]
    AspectRatio,
    #[serde(rename = "assessment")]
    Assessment,
    #[serde(rename = "assignment")]
    Assignment,
    #[serde(rename = "assignment_ind")]
    AssignmentInd,
    #[serde(rename = "assignment_late")]
    AssignmentLate,
    #[serde(rename = "assignment_return")]
    AssignmentReturn,
    #[serde(rename = "assignment_returned")]
    AssignmentReturned,
    #[serde(rename = "assignment_turned_in")]
    AssignmentTurnedIn,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "assistant_direction")]
    AssistantDirection,
    #[serde(rename = "assistant_navigation")]
    AssistantNavigation,
    #[serde(rename = "assistant_photo")]
    AssistantPhoto,
    #[serde(rename = "atm")]
    Atm,
    #[serde(rename = "attach_email")]
    AttachEmail,
    #[serde(rename = "attach_file")]
    AttachFile,
    #[serde(rename = "attach_money")]
    AttachMoney,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "attractions")]
    Attractions,
    #[serde(rename = "attribution")]
    Attribution,
    #[serde(rename = "audiotrack")]
    Audiotrack,
    #[serde(rename = "auto_awesome")]
    AutoAwesome,
    #[serde(rename = "auto_awesome_mosaic")]
    AutoAwesomeMosaic,
    #[serde(rename = "auto_awesome_motion")]
    AutoAwesomeMotion,
    #[serde(rename = "auto_delete")]
    AutoDelete,
    #[serde(rename = "auto_fix_high")]
    AutoFixHigh,
    #[serde(rename = "auto_fix_normal")]
    AutoFixNormal,
    #[serde(rename = "auto_fix_off")]
    AutoFixOff,
    #[serde(rename = "auto_graph")]
    AutoGraph,
    #[serde(rename = "auto_stories")]
    AutoStories,
    #[serde(rename = "autofps_select")]
    AutofpsSelect,
    #[serde(rename = "autorenew")]
    Autorenew,
    #[serde(rename = "av_timer")]
    AvTimer,
    #[serde(rename = "baby_changing_station")]
    BabyChangingStation,
    #[serde(rename = "backpack")]
    Backpack,
    #[serde(rename = "backspace")]
    Backspace,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "backup_table")]
    BackupTable,
    #[serde(rename = "badge")]
    Badge,
    #[serde(rename = "bakery_dining")]
    BakeryDining,
    #[serde(rename = "balcony")]
    Balcony,
    #[serde(rename = "ballot")]
    Ballot,
    #[serde(rename = "bar_chart")]
    BarChart,
    #[serde(rename = "batch_prediction")]
    BatchPrediction,
    #[serde(rename = "bathroom")]
    Bathroom,
    #[serde(rename = "bathtub")]
    Bathtub,
    #[serde(rename = "battery_alert")]
    BatteryAlert,
    #[serde(rename = "battery_charging_full")]
    BatteryChargingFull,
    #[serde(rename = "battery_full")]
    BatteryFull,
    #[serde(rename = "battery_saver")]
    BatterySaver,
    #[serde(rename = "battery_std")]
    BatteryStd,
    #[serde(rename = "battery_unknown")]
    BatteryUnknown,
    #[serde(rename = "beach_access")]
    BeachAccess,
    #[serde(rename = "bed")]
    Bed,
    #[serde(rename = "bedroom_baby")]
    BedroomBaby,
    #[serde(rename = "bedroom_child")]
    BedroomChild,
    #[serde(rename = "bedroom_parent")]
    BedroomParent,
    #[serde(rename = "bedtime")]
    Bedtime,
    #[serde(rename = "beenhere")]
    Beenhere,
    #[serde(rename = "bento")]
    Bento,
    #[serde(rename = "bike_scooter")]
    BikeScooter,
    #[serde(rename = "biotech")]
    Biotech,
    #[serde(rename = "blender")]
    Blender,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "block_flipped")]
    BlockFlipped,
    #[serde(rename = "bloodtype")]
    Bloodtype,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "bluetooth_audio")]
    BluetoothAudio,
    #[serde(rename = "bluetooth_connected")]
    BluetoothConnected,
    #[serde(rename = "bluetooth_disabled")]
    BluetoothDisabled,
    #[serde(rename = "bluetooth_drive")]
    BluetoothDrive,
    #[serde(rename = "bluetooth_searching")]
    BluetoothSearching,
    #[serde(rename = "blur_circular")]
    BlurCircular,
    #[serde(rename = "blur_linear")]
    BlurLinear,
    #[serde(rename = "blur_off")]
    BlurOff,
    #[serde(rename = "blur_on")]
    BlurOn,
    #[serde(rename = "bolt")]
    Bolt,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "book_online")]
    BookOnline,
    #[serde(rename = "bookmark")]
    Bookmark,
    #[serde(rename = "bookmark_add")]
    BookmarkAdd,
    #[serde(rename = "bookmark_added")]
    BookmarkAdded,
    #[serde(rename = "bookmark_border")]
    BookmarkBorder,
    #[serde(rename = "bookmark_outline")]
    BookmarkOutline,
    #[serde(rename = "bookmark_remove")]
    BookmarkRemove,
    #[serde(rename = "bookmarks")]
    Bookmarks,
    #[serde(rename = "border_all")]
    BorderAll,
    #[serde(rename = "border_bottom")]
    BorderBottom,
    #[serde(rename = "border_clear")]
    BorderClear,
    #[serde(rename = "border_color")]
    BorderColor,
    #[serde(rename = "border_horizontal")]
    BorderHorizontal,
    #[serde(rename = "border_inner")]
    BorderInner,
    #[serde(rename = "border_left")]
    BorderLeft,
    #[serde(rename = "border_outer")]
    BorderOuter,
    #[serde(rename = "border_right")]
    BorderRight,
    #[serde(rename = "border_style")]
    BorderStyle,
    #[serde(rename = "border_top")]
    BorderTop,
    #[serde(rename = "border_vertical")]
    BorderVertical,
    #[serde(rename = "branding_watermark")]
    BrandingWatermark,
    #[serde(rename = "breakfast_dining")]
    BreakfastDining,
    #[serde(rename = "brightness_1")]
    Brightness1,
    #[serde(rename = "brightness_2")]
    Brightness2,
    #[serde(rename = "brightness_3")]
    Brightness3,
    #[serde(rename = "brightness_4")]
    Brightness4,
    #[serde(rename = "brightness_5")]
    Brightness5,
    #[serde(rename = "brightness_6")]
    Brightness6,
    #[serde(rename = "brightness_7")]
    Brightness7,
    #[serde(rename = "brightness_auto")]
    BrightnessAuto,
    #[serde(rename = "brightness_high")]
    BrightnessHigh,
    #[serde(rename = "brightness_low")]
    BrightnessLow,
    #[serde(rename = "brightness_medium")]
    BrightnessMedium,
    #[serde(rename = "broken_image")]
    BrokenImage,
    #[serde(rename = "browser_not_supported")]
    BrowserNotSupported,
    #[serde(rename = "brunch_dining")]
    BrunchDining,
    #[serde(rename = "brush")]
    Brush,
    #[serde(rename = "bubble_chart")]
    BubbleChart,
    #[serde(rename = "bug_report")]
    BugReport,
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "build_circle")]
    BuildCircle,
    #[serde(rename = "bungalow")]
    Bungalow,
    #[serde(rename = "burst_mode")]
    BurstMode,
    #[serde(rename = "bus_alert")]
    BusAlert,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "business_center")]
    BusinessCenter,
    #[serde(rename = "cabin")]
    Cabin,
    #[serde(rename = "cable")]
    Cable,
    #[serde(rename = "cached")]
    Cached,
    #[serde(rename = "cake")]
    Cake,
    #[serde(rename = "calculate")]
    Calculate,
    #[serde(rename = "calendar_today")]
    CalendarToday,
    #[serde(rename = "calendar_view_day")]
    CalendarViewDay,
    #[serde(rename = "calendar_view_month")]
    CalendarViewMonth,
    #[serde(rename = "calendar_view_week")]
    CalendarViewWeek,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "call_end")]
    CallEnd,
    #[serde(rename = "call_made")]
    CallMade,
    #[serde(rename = "call_merge")]
    CallMerge,
    #[serde(rename = "call_missed")]
    CallMissed,
    #[serde(rename = "call_missed_outgoing")]
    CallMissedOutgoing,
    #[serde(rename = "call_received")]
    CallReceived,
    #[serde(rename = "call_split")]
    CallSplit,
    #[serde(rename = "call_to_action")]
    CallToAction,
    #[serde(rename = "camera")]
    Camera,
    #[serde(rename = "camera_alt")]
    CameraAlt,
    #[serde(rename = "camera_enhance")]
    CameraEnhance,
    #[serde(rename = "camera_front")]
    CameraFront,
    #[serde(rename = "camera_indoor")]
    CameraIndoor,
    #[serde(rename = "camera_outdoor")]
    CameraOutdoor,
    #[serde(rename = "camera_rear")]
    CameraRear,
    #[serde(rename = "camera_roll")]
    CameraRoll,
    #[serde(rename = "cameraswitch")]
    Cameraswitch,
    #[serde(rename = "campaign")]
    Campaign,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "cancel_presentation")]
    CancelPresentation,
    #[serde(rename = "cancel_schedule_send")]
    CancelScheduleSend,
    #[serde(rename = "car_rental")]
    CarRental,
    #[serde(rename = "car_repair")]
    CarRepair,
    #[serde(rename = "card_giftcard")]
    CardGiftcard,
    #[serde(rename = "card_membership")]
    CardMembership,
    #[serde(rename = "card_travel")]
    CardTravel,
    #[serde(rename = "carpenter")]
    Carpenter,
    #[serde(rename = "cases")]
    Cases,
    #[serde(rename = "casino")]
    Casino,
    #[serde(rename = "cast")]
    Cast,
    #[serde(rename = "cast_connected")]
    CastConnected,
    #[serde(rename = "cast_for_education")]
    CastForEducation,
    #[serde(rename = "catching_pokemon")]
    CatchingPokemon,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "celebration")]
    Celebration,
    #[serde(rename = "cell_wifi")]
    CellWifi,
    #[serde(rename = "center_focus_strong")]
    CenterFocusStrong,
    #[serde(rename = "center_focus_weak")]
    CenterFocusWeak,
    #[serde(rename = "chair")]
    Chair,
    #[serde(rename = "chair_alt")]
    ChairAlt,
    #[serde(rename = "chalet")]
    Chalet,
    #[serde(rename = "change_circle")]
    ChangeCircle,
    #[serde(rename = "change_history")]
    ChangeHistory,
    #[serde(rename = "charging_station")]
    ChargingStation,
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "chat_bubble")]
    ChatBubble,
    #[serde(rename = "chat_bubble_outline")]
    ChatBubbleOutline,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "check_box")]
    CheckBox,
    #[serde(rename = "check_box_outline_blank")]
    CheckBoxOutlineBlank,
    #[serde(rename = "check_circle")]
    CheckCircle,
    #[serde(rename = "check_circle_outline")]
    CheckCircleOutline,
    #[serde(rename = "checklist")]
    Checklist,
    #[serde(rename = "checklist_rtl")]
    ChecklistRtl,
    #[serde(rename = "checkroom")]
    Checkroom,
    #[serde(rename = "chevron_left")]
    ChevronLeft,
    #[serde(rename = "chevron_right")]
    ChevronRight,
    #[serde(rename = "child_care")]
    ChildCare,
    #[serde(rename = "child_friendly")]
    ChildFriendly,
    #[serde(rename = "chrome_reader_mode")]
    ChromeReaderMode,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "circle_notifications")]
    CircleNotifications,
    #[serde(rename = "class_")]
    Class,
    #[serde(rename = "clean_hands")]
    CleanHands,
    #[serde(rename = "cleaning_services")]
    CleaningServices,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "clear_all")]
    ClearAll,
    #[serde(rename = "close")]
    Close,
    #[serde(rename = "close_fullscreen")]
    CloseFullscreen,
    #[serde(rename = "closed_caption")]
    ClosedCaption,
    #[serde(rename = "closed_caption_disabled")]
    ClosedCaptionDisabled,
    #[serde(rename = "closed_caption_off")]
    ClosedCaptionOff,
    #[serde(rename = "cloud")]
    Cloud,
    #[serde(rename = "cloud_circle")]
    CloudCircle,
    #[serde(rename = "cloud_done")]
    CloudDone,
    #[serde(rename = "cloud_download")]
    CloudDownload,
    #[serde(rename = "cloud_off")]
    CloudOff,
    #[serde(rename = "cloud_queue")]
    CloudQueue,
    #[serde(rename = "cloud_upload")]
    CloudUpload,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "code_off")]
    CodeOff,
    #[serde(rename = "coffee")]
    Coffee,
    #[serde(rename = "coffee_maker")]
    CoffeeMaker,
    #[serde(rename = "collections")]
    Collections,
    #[serde(rename = "collections_bookmark")]
    CollectionsBookmark,
    #[serde(rename = "color_lens")]
    ColorLens,
    #[serde(rename = "colorize")]
    Colorize,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "comment_bank")]
    CommentBank,
    #[serde(rename = "commute")]
    Commute,
    #[serde(rename = "compare")]
    Compare,
    #[serde(rename = "compare_arrows")]
    CompareArrows,
    #[serde(rename = "compass_calibration")]
    CompassCalibration,
    #[serde(rename = "compress")]
    Compress,
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "confirmation_num")]
    ConfirmationNum,
    #[serde(rename = "confirmation_number")]
    ConfirmationNumber,
    #[serde(rename = "connect_without_contact")]
    ConnectWithoutContact,
    #[serde(rename = "connected_tv")]
    ConnectedTv,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "contact_mail")]
    ContactMail,
    #[serde(rename = "contact_page")]
    ContactPage,
    #[serde(rename = "contact_phone")]
    ContactPhone,
    #[serde(rename = "contact_support")]
    ContactSupport,
    #[serde(rename = "contactless")]
    Contactless,
    #[serde(rename = "contacts")]
    Contacts,
    #[serde(rename = "content_copy")]
    ContentCopy,
    #[serde(rename = "content_cut")]
    ContentCut,
    #[serde(rename = "content_paste")]
    ContentPaste,
    #[serde(rename = "content_paste_off")]
    ContentPasteOff,
    #[serde(rename = "control_camera")]
    ControlCamera,
    #[serde(rename = "control_point")]
    ControlPoint,
    #[serde(rename = "control_point_duplicate")]
    ControlPointDuplicate,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "copy_all")]
    CopyAll,
    #[serde(rename = "copyright")]
    Copyright,
    #[serde(rename = "coronavirus")]
    Coronavirus,
    #[serde(rename = "corporate_fare")]
    CorporateFare,
    #[serde(rename = "cottage")]
    Cottage,
    #[serde(rename = "countertops")]
    Countertops,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "create_new_folder")]
    CreateNewFolder,
    #[serde(rename = "credit_card")]
    CreditCard,
    #[serde(rename = "credit_card_off")]
    CreditCardOff,
    #[serde(rename = "credit_score")]
    CreditScore,
    #[serde(rename = "crib")]
    Crib,
    #[serde(rename = "crop")]
    Crop,
    #[serde(rename = "crop_16_9")]
    Crop169,
    #[serde(rename = "crop_3_2")]
    Crop32,
    #[serde(rename = "crop_5_4")]
    Crop54,
    #[serde(rename = "crop_7_5")]
    Crop75,
    #[serde(rename = "crop_din")]
    CropDin,
    #[serde(rename = "crop_free")]
    CropFree,
    #[serde(rename = "crop_landscape")]
    CropLandscape,
    #[serde(rename = "crop_original")]
    CropOriginal,
    #[serde(rename = "crop_portrait")]
    CropPortrait,
    #[serde(rename = "crop_rotate")]
    CropRotate,
    #[serde(rename = "crop_square")]
    CropSquare,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "dangerous")]
    Dangerous,
    #[serde(rename = "dark_mode")]
    DarkMode,
    #[serde(rename = "dashboard")]
    Dashboard,
    #[serde(rename = "dashboard_customize")]
    DashboardCustomize,
    #[serde(rename = "data_saver_off")]
    DataSaverOff,
    #[serde(rename = "data_saver_on")]
    DataSaverOn,
    #[serde(rename = "data_usage")]
    DataUsage,
    #[serde(rename = "date_range")]
    DateRange,
    #[serde(rename = "deck")]
    Deck,
    #[serde(rename = "dehaze")]
    Dehaze,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "delete_forever")]
    DeleteForever,
    #[serde(rename = "delete_outline")]
    DeleteOutline,
    #[serde(rename = "delete_sweep")]
    DeleteSweep,
    #[serde(rename = "delivery_dining")]
    DeliveryDining,
    #[serde(rename = "departure_board")]
    DepartureBoard,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "design_services")]
    DesignServices,
    #[serde(rename = "desktop_access_disabled")]
    DesktopAccessDisabled,
    #[serde(rename = "desktop_mac")]
    DesktopMac,
    #[serde(rename = "desktop_windows")]
    DesktopWindows,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "developer_board")]
    DeveloperBoard,
    #[serde(rename = "developer_board_off")]
    DeveloperBoardOff,
    #[serde(rename = "developer_mode")]
    DeveloperMode,
    #[serde(rename = "device_hub")]
    DeviceHub,
    #[serde(rename = "device_thermostat")]
    DeviceThermostat,
    #[serde(rename = "device_unknown")]
    DeviceUnknown,
    #[serde(rename = "devices")]
    Devices,
    #[serde(rename = "devices_other")]
    DevicesOther,
    #[serde(rename = "dialer_sip")]
    DialerSip,
    #[serde(rename = "dialpad")]
    Dialpad,
    #[serde(rename = "dining")]
    Dining,
    #[serde(rename = "dinner_dining")]
    DinnerDining,
    #[serde(rename = "directions")]
    Directions,
    #[serde(rename = "directions_bike")]
    DirectionsBike,
    #[serde(rename = "directions_boat")]
    DirectionsBoat,
    #[serde(rename = "directions_boat_filled")]
    DirectionsBoatFilled,
    #[serde(rename = "directions_bus")]
    DirectionsBus,
    #[serde(rename = "directions_bus_filled")]
    DirectionsBusFilled,
    #[serde(rename = "directions_car")]
    DirectionsCar,
    #[serde(rename = "directions_car_filled")]
    DirectionsCarFilled,
    #[serde(rename = "directions_ferry")]
    DirectionsFerry,
    #[serde(rename = "directions_off")]
    DirectionsOff,
    #[serde(rename = "directions_railway_filled")]
    DirectionsRailwayFilled,
    #[serde(rename = "directions_run")]
    DirectionsRun,
    #[serde(rename = "directions_railway")]
    DirectionsRailway,
    #[serde(rename = "directions_subway")]
    DirectionsSubway,
    #[serde(rename = "directions_subway_filled")]
    DirectionsSubwayFilled,
    #[serde(rename = "directions_train")]
    DirectionsTrain,
    #[serde(rename = "directions_transit")]
    DirectionsTransit,
    #[serde(rename = "directions_transit_filled")]
    DirectionsTransitFilled,
    #[serde(rename = "directions_walk")]
    DirectionsWalk,
    #[serde(rename = "dirty_lens")]
    DirtyLens,
    #[serde(rename = "disabled_by_default")]
    DisabledByDefault,
    #[serde(rename = "disc_full")]
    DiscFull,
    #[serde(rename = "dnd_forwardslash")]
    DndForwardslash,
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "do_disturb")]
    DoDisturb,
    #[serde(rename = "do_disturb_alt")]
    DoDisturbAlt,
    #[serde(rename = "do_disturb_off")]
    DoDisturbOff,
    #[serde(rename = "do_disturb_on")]
    DoDisturbOn,
    #[serde(rename = "do_not_disturb")]
    DoNotDisturb,
    #[serde(rename = "do_not_disturb_alt")]
    DoNotDisturbAlt,
    #[serde(rename = "do_not_disturb_off")]
    DoNotDisturbOff,
    #[serde(rename = "do_not_disturb_on")]
    DoNotDisturbOn,
    #[serde(rename = "do_not_disturb_on_total_silence")]
    DoNotDisturbOnTotalSilence,
    #[serde(rename = "do_not_step")]
    DoNotStep,
    #[serde(rename = "do_not_touch")]
    DoNotTouch,
    #[serde(rename = "dock")]
    Dock,
    #[serde(rename = "document_scanner")]
    DocumentScanner,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "domain_disabled")]
    DomainDisabled,
    #[serde(rename = "domain_verification")]
    DomainVerification,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "done_all")]
    DoneAll,
    #[serde(rename = "done_outline")]
    DoneOutline,
    #[serde(rename = "donut_large")]
    DonutLarge,
    #[serde(rename = "donut_small")]
    DonutSmall,
    #[serde(rename = "door_back")]
    DoorBack,
    #[serde(rename = "door_front")]
    DoorFront,
    #[serde(rename = "door_sliding")]
    DoorSliding,
    #[serde(rename = "doorbell")]
    Doorbell,
    #[serde(rename = "double_arrow")]
    DoubleArrow,
    #[serde(rename = "downhill_skiing")]
    DownhillSkiing,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "download_done")]
    DownloadDone,
    #[serde(rename = "download_for_offline")]
    DownloadForOffline,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "drafts")]
    Drafts,
    #[serde(rename = "drag_handle")]
    DragHandle,
    #[serde(rename = "drag_indicator")]
    DragIndicator,
    #[serde(rename = "drive_eta")]
    DriveEta,
    #[serde(rename = "drive_file_move")]
    DriveFileMove,
    #[serde(rename = "drive_file_move_outline")]
    DriveFileMoveOutline,
    #[serde(rename = "drive_file_rename_outline")]
    DriveFileRenameOutline,
    #[serde(rename = "drive_folder_upload")]
    DriveFolderUpload,
    #[serde(rename = "dry")]
    Dry,
    #[serde(rename = "dry_cleaning")]
    DryCleaning,
    #[serde(rename = "duo")]
    Duo,
    #[serde(rename = "dvr")]
    Dvr,
    #[serde(rename = "dynamic_feed")]
    DynamicFeed,
    #[serde(rename = "dynamic_form")]
    DynamicForm,
    #[serde(rename = "e_mobiledata")]
    EMobiledata,
    #[serde(rename = "earbuds")]
    Earbuds,
    #[serde(rename = "earbuds_battery")]
    EarbudsBattery,
    #[serde(rename = "east")]
    East,
    #[serde(rename = "eco")]
    Eco,
    #[serde(rename = "edgesensor_high")]
    EdgesensorHigh,
    #[serde(rename = "edgesensor_low")]
    EdgesensorLow,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "edit_attributes")]
    EditAttributes,
    #[serde(rename = "edit_location")]
    EditLocation,
    #[serde(rename = "edit_location_alt")]
    EditLocationAlt,
    #[serde(rename = "edit_notifications")]
    EditNotifications,
    #[serde(rename = "edit_off")]
    EditOff,
    #[serde(rename = "edit_road")]
    EditRoad,
    #[serde(rename = "eight_k")]
    EightK,
    #[serde(rename = "eight_k_plus")]
    EightKPlus,
    #[serde(rename = "eight_mp")]
    EightMp,
    #[serde(rename = "eighteen_mp")]
    EighteenMp,
    #[serde(rename = "eject")]
    Eject,
    #[serde(rename = "elderly")]
    Elderly,
    #[serde(rename = "electric_bike")]
    ElectricBike,
    #[serde(rename = "electric_car")]
    ElectricCar,
    #[serde(rename = "electric_moped")]
    ElectricMoped,
    #[serde(rename = "electric_rickshaw")]
    ElectricRickshaw,
    #[serde(rename = "electric_scooter")]
    ElectricScooter,
    #[serde(rename = "electrical_services")]
    ElectricalServices,
    #[serde(rename = "elevator")]
    Elevator,
    #[serde(rename = "eleven_mp")]
    ElevenMp,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "emoji_emotions")]
    EmojiEmotions,
    #[serde(rename = "emoji_events")]
    EmojiEvents,
    #[serde(rename = "emoji_flags")]
    EmojiFlags,
    #[serde(rename = "emoji_food_beverage")]
    EmojiFoodBeverage,
    #[serde(rename = "emoji_nature")]
    EmojiNature,
    #[serde(rename = "emoji_objects")]
    EmojiObjects,
    #[serde(rename = "emoji_people")]
    EmojiPeople,
    #[serde(rename = "emoji_symbols")]
    EmojiSymbols,
    #[serde(rename = "emoji_transportation")]
    EmojiTransportation,
    #[serde(rename = "engineering")]
    Engineering,
    #[serde(rename = "enhance_photo_translate")]
    EnhancePhotoTranslate,
    #[serde(rename = "enhanced_encryption")]
    EnhancedEncryption,
    #[serde(rename = "equalizer")]
    Equalizer,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "error_outline")]
    ErrorOutline,
    #[serde(rename = "escalator")]
    Escalator,
    #[serde(rename = "escalator_warning")]
    EscalatorWarning,
    #[serde(rename = "euro")]
    Euro,
    #[serde(rename = "euro_symbol")]
    EuroSymbol,
    #[serde(rename = "ev_station")]
    EvStation,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "event_available")]
    EventAvailable,
    #[serde(rename = "event_busy")]
    EventBusy,
    #[serde(rename = "event_note")]
    EventNote,
    #[serde(rename = "event_seat")]
    EventSeat,
    #[serde(rename = "exit_to_app")]
    ExitToApp,
    #[serde(rename = "expand")]
    Expand,
    #[serde(rename = "expand_less")]
    ExpandLess,
    #[serde(rename = "expand_more")]
    ExpandMore,
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "explore")]
    Explore,
    #[serde(rename = "explore_off")]
    ExploreOff,
    #[serde(rename = "exposure")]
    Exposure,
    #[serde(rename = "exposure_minus_1")]
    ExposureMinus1,
    #[serde(rename = "exposure_minus_2")]
    ExposureMinus2,
    #[serde(rename = "exposure_neg_1")]
    ExposureNeg1,
    #[serde(rename = "exposure_neg_2")]
    ExposureNeg2,
    #[serde(rename = "exposure_plus_1")]
    ExposurePlus1,
    #[serde(rename = "exposure_plus_2")]
    ExposurePlus2,
    #[serde(rename = "exposure_zero")]
    ExposureZero,
    #[serde(rename = "extension")]
    Extension,
    #[serde(rename = "extension_off")]
    ExtensionOff,
    #[serde(rename = "face")]
    Face,
    #[serde(rename = "face_retouching_off")]
    FaceRetouchingOff,
    #[serde(rename = "face_retouching_natural")]
    FaceRetouchingNatural,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "fact_check")]
    FactCheck,
    #[serde(rename = "family_restroom")]
    FamilyRestroom,
    #[serde(rename = "fast_forward")]
    FastForward,
    #[serde(rename = "fast_rewind")]
    FastRewind,
    #[serde(rename = "fastfood")]
    Fastfood,
    #[serde(rename = "favorite")]
    Favorite,
    #[serde(rename = "favorite_border")]
    FavoriteBorder,
    #[serde(rename = "favorite_outline")]
    FavoriteOutline,
    #[serde(rename = "featured_play_list")]
    FeaturedPlayList,
    #[serde(rename = "featured_video")]
    FeaturedVideo,
    #[serde(rename = "feed")]
    Feed,
    #[serde(rename = "feedback")]
    Feedback,
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "fence")]
    Fence,
    #[serde(rename = "festival")]
    Festival,
    #[serde(rename = "fiber_dvr")]
    FiberDvr,
    #[serde(rename = "fiber_manual_record")]
    FiberManualRecord,
    #[serde(rename = "fiber_new")]
    FiberNew,
    #[serde(rename = "fiber_pin")]
    FiberPin,
    #[serde(rename = "fiber_smart_record")]
    FiberSmartRecord,
    #[serde(rename = "fifteen_mp")]
    FifteenMp,
    #[serde(rename = "file_copy")]
    FileCopy,
    #[serde(rename = "file_download")]
    FileDownload,
    #[serde(rename = "file_download_done")]
    FileDownloadDone,
    #[serde(rename = "file_download_off")]
    FileDownloadOff,
    #[serde(rename = "file_present")]
    FilePresent,
    #[serde(rename = "file_upload")]
    FileUpload,
    #[serde(rename = "filter")]
    Filter,
    #[serde(rename = "filter_1")]
    Filter1,
    #[serde(rename = "filter_2")]
    Filter2,
    #[serde(rename = "filter_3")]
    Filter3,
    #[serde(rename = "filter_4")]
    Filter4,
    #[serde(rename = "filter_5")]
    Filter5,
    #[serde(rename = "filter_6")]
    Filter6,
    #[serde(rename = "filter_7")]
    Filter7,
    #[serde(rename = "filter_8")]
    Filter8,
    #[serde(rename = "filter_9")]
    Filter9,
    #[serde(rename = "filter_9_plus")]
    Filter9Plus,
    #[serde(rename = "filter_alt")]
    FilterAlt,
    #[serde(rename = "filter_b_and_w")]
    FilterBAndW,
    #[serde(rename = "filter_center_focus")]
    FilterCenterFocus,
    #[serde(rename = "filter_drama")]
    FilterDrama,
    #[serde(rename = "filter_frames")]
    FilterFrames,
    #[serde(rename = "filter_hdr")]
    FilterHdr,
    #[serde(rename = "filter_list")]
    FilterList,
    #[serde(rename = "filter_list_alt")]
    FilterListAlt,
    #[serde(rename = "filter_none")]
    FilterNone,
    #[serde(rename = "filter_tilt_shift")]
    FilterTiltShift,
    #[serde(rename = "filter_vintage")]
    FilterVintage,
    #[serde(rename = "find_in_page")]
    FindInPage,
    #[serde(rename = "find_replace")]
    FindReplace,
    #[serde(rename = "fingerprint")]
    Fingerprint,
    #[serde(rename = "fire_extinguisher")]
    FireExtinguisher,
    #[serde(rename = "fire_hydrant")]
    FireHydrant,
    #[serde(rename = "fireplace")]
    Fireplace,
    #[serde(rename = "first_page")]
    FirstPage,
    #[serde(rename = "fit_screen")]
    FitScreen,
    #[serde(rename = "fitness_center")]
    FitnessCenter,
    #[serde(rename = "five_g")]
    FiveG,
    #[serde(rename = "five_k")]
    FiveK,
    #[serde(rename = "five_k_plus")]
    FiveKPlus,
    #[serde(rename = "five_mp")]
    FiveMp,
    #[serde(rename = "flag")]
    Flag,
    #[serde(rename = "flaky")]
    Flaky,
    #[serde(rename = "flare")]
    Flare,
    #[serde(rename = "flash_auto")]
    FlashAuto,
    #[serde(rename = "flash_off")]
    FlashOff,
    #[serde(rename = "flash_on")]
    FlashOn,
    #[serde(rename = "flashlight_off")]
    FlashlightOff,
    #[serde(rename = "flashlight_on")]
    FlashlightOn,
    #[serde(rename = "flatware")]
    Flatware,
    #[serde(rename = "flight")]
    Flight,
    #[serde(rename = "flight_land")]
    FlightLand,
    #[serde(rename = "flight_takeoff")]
    FlightTakeoff,
    #[serde(rename = "flip")]
    Flip,
    #[serde(rename = "flip_camera_android")]
    FlipCameraAndroid,
    #[serde(rename = "flip_camera_ios")]
    FlipCameraIos,
    #[serde(rename = "flip_to_back")]
    FlipToBack,
    #[serde(rename = "flip_to_front")]
    FlipToFront,
    #[serde(rename = "flourescent")]
    Flourescent,
    #[serde(rename = "flutter_dash")]
    FlutterDash,
    #[serde(rename = "fmd_bad")]
    FmdBad,
    #[serde(rename = "fmd_good")]
    FmdGood,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "folder_open")]
    FolderOpen,
    #[serde(rename = "folder_shared")]
    FolderShared,
    #[serde(rename = "folder_special")]
    FolderSpecial,
    #[serde(rename = "follow_the_signs")]
    FollowTheSigns,
    #[serde(rename = "font_download")]
    FontDownload,
    #[serde(rename = "font_download_off")]
    FontDownloadOff,
    #[serde(rename = "food_bank")]
    FoodBank,
    #[serde(rename = "format_align_center")]
    FormatAlignCenter,
    #[serde(rename = "format_align_justify")]
    FormatAlignJustify,
    #[serde(rename = "format_align_left")]
    FormatAlignLeft,
    #[serde(rename = "format_align_right")]
    FormatAlignRight,
    #[serde(rename = "format_bold")]
    FormatBold,
    #[serde(rename = "format_clear")]
    FormatClear,
    #[serde(rename = "format_color_fill")]
    FormatColorFill,
    #[serde(rename = "format_color_reset")]
    FormatColorReset,
    #[serde(rename = "format_color_text")]
    FormatColorText,
    #[serde(rename = "format_indent_decrease")]
    FormatIndentDecrease,
    #[serde(rename = "format_indent_increase")]
    FormatIndentIncrease,
    #[serde(rename = "format_italic")]
    FormatItalic,
    #[serde(rename = "format_line_spacing")]
    FormatLineSpacing,
    #[serde(rename = "format_list_bulleted")]
    FormatListBulleted,
    #[serde(rename = "format_list_numbered")]
    FormatListNumbered,
    #[serde(rename = "format_list_numbered_rtl")]
    FormatListNumberedRtl,
    #[serde(rename = "format_paint")]
    FormatPaint,
    #[serde(rename = "format_quote")]
    FormatQuote,
    #[serde(rename = "format_shapes")]
    FormatShapes,
    #[serde(rename = "format_size")]
    FormatSize,
    #[serde(rename = "format_strikethrough")]
    FormatStrikethrough,
    #[serde(rename = "format_textdirection_l_to_r")]
    FormatTextdirectionLToR,
    #[serde(rename = "format_textdirection_r_to_l")]
    FormatTextdirectionRToL,
    #[serde(rename = "format_underline")]
    FormatUnderline,
    #[serde(rename = "format_underlined")]
    FormatUnderlined,
    #[serde(rename = "forum")]
    Forum,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "forward_10")]
    Forward10,
    #[serde(rename = "forward_30")]
    Forward30,
    #[serde(rename = "forward_5")]
    Forward5,
    #[serde(rename = "forward_to_inbox")]
    ForwardToInbox,
    #[serde(rename = "foundation")]
    Foundation,
    #[serde(rename = "four_g_mobiledata")]
    FourGMobiledata,
    #[serde(rename = "four_g_plus_mobiledata")]
    FourGPlusMobiledata,
    #[serde(rename = "four_k")]
    FourK,
    #[serde(rename = "four_k_plus")]
    FourKPlus,
    #[serde(rename = "four_mp")]
    FourMp,
    #[serde(rename = "fourteen_mp")]
    FourteenMp,
    #[serde(rename = "free_breakfast")]
    FreeBreakfast,
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "fullscreen_exit")]
    FullscreenExit,
    #[serde(rename = "functions")]
    Functions,
    #[serde(rename = "g_mobiledata")]
    GMobiledata,
    #[serde(rename = "g_translate")]
    GTranslate,
    #[serde(rename = "gamepad")]
    Gamepad,
    #[serde(rename = "games")]
    Games,
    #[serde(rename = "garage")]
    Garage,
    #[serde(rename = "gavel")]
    Gavel,
    #[serde(rename = "gesture")]
    Gesture,
    #[serde(rename = "get_app")]
    GetApp,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "gite")]
    Gite,
    #[serde(rename = "golf_course")]
    GolfCourse,
    #[serde(rename = "gpp_bad")]
    GppBad,
    #[serde(rename = "gpp_good")]
    GppGood,
    #[serde(rename = "gpp_maybe")]
    GppMaybe,
    #[serde(rename = "gps_fixed")]
    GpsFixed,
    #[serde(rename = "gps_not_fixed")]
    GpsNotFixed,
    #[serde(rename = "gps_off")]
    GpsOff,
    #[serde(rename = "grade")]
    Grade,
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "grading")]
    Grading,
    #[serde(rename = "grain")]
    Grain,
    #[serde(rename = "graphic_eq")]
    GraphicEq,
    #[serde(rename = "grass")]
    Grass,
    #[serde(rename = "grid_3x3")]
    Grid3x3,
    #[serde(rename = "grid_4x4")]
    Grid4x4,
    #[serde(rename = "grid_goldenratio")]
    GridGoldenratio,
    #[serde(rename = "grid_off")]
    GridOff,
    #[serde(rename = "grid_on")]
    GridOn,
    #[serde(rename = "grid_view")]
    GridView,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "group_add")]
    GroupAdd,
    #[serde(rename = "group_work")]
    GroupWork,
    #[serde(rename = "groups")]
    Groups,
    #[serde(rename = "h_mobiledata")]
    HMobiledata,
    #[serde(rename = "h_plus_mobiledata")]
    HPlusMobiledata,
    #[serde(rename = "hail")]
    Hail,
    #[serde(rename = "handyman")]
    Handyman,
    #[serde(rename = "hardware")]
    Hardware,
    #[serde(rename = "hd")]
    Hd,
    #[serde(rename = "hdr_auto")]
    HdrAuto,
    #[serde(rename = "hdr_auto_select")]
    HdrAutoSelect,
    #[serde(rename = "hdr_enhanced_select")]
    HdrEnhancedSelect,
    #[serde(rename = "hdr_off")]
    HdrOff,
    #[serde(rename = "hdr_off_select")]
    HdrOffSelect,
    #[serde(rename = "hdr_on")]
    HdrOn,
    #[serde(rename = "hdr_on_select")]
    HdrOnSelect,
    #[serde(rename = "hdr_plus")]
    HdrPlus,
    #[serde(rename = "hdr_strong")]
    HdrStrong,
    #[serde(rename = "hdr_weak")]
    HdrWeak,
    #[serde(rename = "headphones")]
    Headphones,
    #[serde(rename = "headphones_battery")]
    HeadphonesBattery,
    #[serde(rename = "headset")]
    Headset,
    #[serde(rename = "headset_mic")]
    HeadsetMic,
    #[serde(rename = "headset_off")]
    HeadsetOff,
    #[serde(rename = "healing")]
    Healing,
    #[serde(rename = "health_and_safety")]
    HealthAndSafety,
    #[serde(rename = "hearing")]
    Hearing,
    #[serde(rename = "hearing_disabled")]
    HearingDisabled,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "help")]
    Help,
    #[serde(rename = "help_center")]
    HelpCenter,
    #[serde(rename = "help_outline")]
    HelpOutline,
    #[serde(rename = "hevc")]
    Hevc,
    #[serde(rename = "hide_image")]
    HideImage,
    #[serde(rename = "hide_source")]
    HideSource,
    #[serde(rename = "high_quality")]
    HighQuality,
    #[serde(rename = "highlight")]
    Highlight,
    #[serde(rename = "highlight_alt")]
    HighlightAlt,
    #[serde(rename = "highlight_off")]
    HighlightOff,
    #[serde(rename = "highlight_remove")]
    HighlightRemove,
    #[serde(rename = "hiking")]
    Hiking,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "history_edu")]
    HistoryEdu,
    #[serde(rename = "history_toggle_off")]
    HistoryToggleOff,
    #[serde(rename = "holiday_village")]
    HolidayVillage,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "home_filled")]
    HomeFilled,
    #[serde(rename = "home_max")]
    HomeMax,
    #[serde(rename = "home_mini")]
    HomeMini,
    #[serde(rename = "home_repair_service")]
    HomeRepairService,
    #[serde(rename = "home_work")]
    HomeWork,
    #[serde(rename = "horizontal_distribute")]
    HorizontalDistribute,
    #[serde(rename = "horizontal_rule")]
    HorizontalRule,
    #[serde(rename = "horizontal_split")]
    HorizontalSplit,
    #[serde(rename = "hot_tub")]
    HotTub,
    #[serde(rename = "hotel")]
    Hotel,
    #[serde(rename = "hourglass_bottom")]
    HourglassBottom,
    #[serde(rename = "hourglass_disabled")]
    HourglassDisabled,
    #[serde(rename = "hourglass_empty")]
    HourglassEmpty,
    #[serde(rename = "hourglass_full")]
    HourglassFull,
    #[serde(rename = "hourglass_top")]
    HourglassTop,
    #[serde(rename = "house")]
    House,
    #[serde(rename = "house_siding")]
    HouseSiding,
    #[serde(rename = "houseboat")]
    Houseboat,
    #[serde(rename = "how_to_reg")]
    HowToReg,
    #[serde(rename = "how_to_vote")]
    HowToVote,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "hvac")]
    Hvac,
    #[serde(rename = "ice_skating")]
    IceSkating,
    #[serde(rename = "icecream")]
    Icecream,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "image_aspect_ratio")]
    ImageAspectRatio,
    #[serde(rename = "image_not_supported")]
    ImageNotSupported,
    #[serde(rename = "image_search")]
    ImageSearch,
    #[serde(rename = "imagesearch_roller")]
    ImagesearchRoller,
    #[serde(rename = "import_contacts")]
    ImportContacts,
    #[serde(rename = "import_export")]
    ImportExport,
    #[serde(rename = "important_devices")]
    ImportantDevices,
    #[serde(rename = "inbox")]
    Inbox,
    #[serde(rename = "indeterminate_check_box")]
    IndeterminateCheckBox,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "info_outline")]
    InfoOutline,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "insert_chart")]
    InsertChart,
    #[serde(rename = "insert_comment")]
    InsertComment,
    #[serde(rename = "insert_drive_file")]
    InsertDriveFile,
    #[serde(rename = "insert_emoticon")]
    InsertEmoticon,
    #[serde(rename = "insert_invitation")]
    InsertInvitation,
    #[serde(rename = "insert_link")]
    InsertLink,
    #[serde(rename = "insert_photo")]
    InsertPhoto,
    #[serde(rename = "insights")]
    Insights,
    #[serde(rename = "integration_instructions")]
    IntegrationInstructions,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "inventory_2")]
    Inventory2,
    #[serde(rename = "invert_colors")]
    InvertColors,
    #[serde(rename = "invert_colors_off")]
    InvertColorsOff,
    #[serde(rename = "invert_colors_on")]
    InvertColorsOn,
    #[serde(rename = "ios_share")]
    IosShare,
    #[serde(rename = "iron")]
    Iron,
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "kayaking")]
    Kayaking,
    #[serde(rename = "keyboard")]
    Keyboard,
    #[serde(rename = "keyboard_alt")]
    KeyboardAlt,
    #[serde(rename = "keyboard_arrow_down")]
    KeyboardArrowDown,
    #[serde(rename = "keyboard_arrow_left")]
    KeyboardArrowLeft,
    #[serde(rename = "keyboard_arrow_right")]
    KeyboardArrowRight,
    #[serde(rename = "keyboard_arrow_up")]
    KeyboardArrowUp,
    #[serde(rename = "keyboard_backspace")]
    KeyboardBackspace,
    #[serde(rename = "keyboard_capslock")]
    KeyboardCapslock,
    #[serde(rename = "keyboard_control")]
    KeyboardControl,
    #[serde(rename = "keyboard_hide")]
    KeyboardHide,
    #[serde(rename = "keyboard_return")]
    KeyboardReturn,
    #[serde(rename = "keyboard_tab")]
    KeyboardTab,
    #[serde(rename = "keyboard_voice")]
    KeyboardVoice,
    #[serde(rename = "king_bed")]
    KingBed,
    #[serde(rename = "kitchen")]
    Kitchen,
    #[serde(rename = "kitesurfing")]
    Kitesurfing,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "label_important")]
    LabelImportant,
    #[serde(rename = "label_important_outline")]
    LabelImportantOutline,
    #[serde(rename = "label_off")]
    LabelOff,
    #[serde(rename = "label_outline")]
    LabelOutline,
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "laptop")]
    Laptop,
    #[serde(rename = "laptop_chromebook")]
    LaptopChromebook,
    #[serde(rename = "laptop_mac")]
    LaptopMac,
    #[serde(rename = "laptop_windows")]
    LaptopWindows,
    #[serde(rename = "last_page")]
    LastPage,
    #[serde(rename = "launch")]
    Launch,
    #[serde(rename = "layers")]
    Layers,
    #[serde(rename = "layers_clear")]
    LayersClear,
    #[serde(rename = "leaderboard")]
    Leaderboard,
    #[serde(rename = "leak_add")]
    LeakAdd,
    #[serde(rename = "leak_remove")]
    LeakRemove,
    #[serde(rename = "leave_bags_at_home")]
    LeaveBagsAtHome,
    #[serde(rename = "legend_toggle")]
    LegendToggle,
    #[serde(rename = "lens")]
    Lens,
    #[serde(rename = "lens_blur")]
    LensBlur,
    #[serde(rename = "library_add")]
    LibraryAdd,
    #[serde(rename = "library_add_check")]
    LibraryAddCheck,
    #[serde(rename = "library_books")]
    LibraryBooks,
    #[serde(rename = "library_music")]
    LibraryMusic,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "light_mode")]
    LightMode,
    #[serde(rename = "lightbulb")]
    Lightbulb,
    #[serde(rename = "lightbulb_outline")]
    LightbulbOutline,
    #[serde(rename = "line_style")]
    LineStyle,
    #[serde(rename = "line_weight")]
    LineWeight,
    #[serde(rename = "linear_scale")]
    LinearScale,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "link_off")]
    LinkOff,
    #[serde(rename = "linked_camera")]
    LinkedCamera,
    #[serde(rename = "liquor")]
    Liquor,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "list_alt")]
    ListAlt,
    #[serde(rename = "live_help")]
    LiveHelp,
    #[serde(rename = "live_tv")]
    LiveTv,
    #[serde(rename = "living")]
    Living,
    #[serde(rename = "local_activity")]
    LocalActivity,
    #[serde(rename = "local_airport")]
    LocalAirport,
    #[serde(rename = "local_atm")]
    LocalAtm,
    #[serde(rename = "local_attraction")]
    LocalAttraction,
    #[serde(rename = "local_bar")]
    LocalBar,
    #[serde(rename = "local_cafe")]
    LocalCafe,
    #[serde(rename = "local_car_wash")]
    LocalCarWash,
    #[serde(rename = "local_convenience_store")]
    LocalConvenienceStore,
    #[serde(rename = "local_dining")]
    LocalDining,
    #[serde(rename = "local_drink")]
    LocalDrink,
    #[serde(rename = "local_fire_department")]
    LocalFireDepartment,
    #[serde(rename = "local_florist")]
    LocalFlorist,
    #[serde(rename = "local_gas_station")]
    LocalGasStation,
    #[serde(rename = "local_grocery_store")]
    LocalGroceryStore,
    #[serde(rename = "local_hospital")]
    LocalHospital,
    #[serde(rename = "local_hotel")]
    LocalHotel,
    #[serde(rename = "local_laundry_service")]
    LocalLaundryService,
    #[serde(rename = "local_library")]
    LocalLibrary,
    #[serde(rename = "local_mall")]
    LocalMall,
    #[serde(rename = "local_movies")]
    LocalMovies,
    #[serde(rename = "local_offer")]
    LocalOffer,
    #[serde(rename = "local_parking")]
    LocalParking,
    #[serde(rename = "local_pharmacy")]
    LocalPharmacy,
    #[serde(rename = "local_phone")]
    LocalPhone,
    #[serde(rename = "local_pizza")]
    LocalPizza,
    #[serde(rename = "local_play")]
    LocalPlay,
    #[serde(rename = "local_police")]
    LocalPolice,
    #[serde(rename = "local_post_office")]
    LocalPostOffice,
    #[serde(rename = "local_print_shop")]
    LocalPrintShop,
    #[serde(rename = "local_printshop")]
    LocalPrintshop,
    #[serde(rename = "local_restaurant")]
    LocalRestaurant,
    #[serde(rename = "local_see")]
    LocalSee,
    #[serde(rename = "local_shipping")]
    LocalShipping,
    #[serde(rename = "local_taxi")]
    LocalTaxi,
    #[serde(rename = "location_city")]
    LocationCity,
    #[serde(rename = "location_disabled")]
    LocationDisabled,
    #[serde(rename = "location_history")]
    LocationHistory,
    #[serde(rename = "location_off")]
    LocationOff,
    #[serde(rename = "location_on")]
    LocationOn,
    #[serde(rename = "location_pin")]
    LocationPin,
    #[serde(rename = "location_searching")]
    LocationSearching,
    #[serde(rename = "lock")]
    Lock,
    #[serde(rename = "lock_clock")]
    LockClock,
    #[serde(rename = "lock_open")]
    LockOpen,
    #[serde(rename = "lock_outline")]
    LockOutline,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "logout")]
    Logout,
    #[serde(rename = "looks")]
    Looks,
    #[serde(rename = "looks_3")]
    Looks3,
    #[serde(rename = "looks_4")]
    Looks4,
    #[serde(rename = "looks_5")]
    Looks5,
    #[serde(rename = "looks_6")]
    Looks6,
    #[serde(rename = "looks_one")]
    LooksOne,
    #[serde(rename = "looks_two")]
    LooksTwo,
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "loupe")]
    Loupe,
    #[serde(rename = "low_priority")]
    LowPriority,
    #[serde(rename = "loyalty")]
    Loyalty,
    #[serde(rename = "lte_mobiledata")]
    LteMobiledata,
    #[serde(rename = "lte_plus_mobiledata")]
    LtePlusMobiledata,
    #[serde(rename = "luggage")]
    Luggage,
    #[serde(rename = "lunch_dining")]
    LunchDining,
    #[serde(rename = "mail")]
    Mail,
    #[serde(rename = "mail_outline")]
    MailOutline,
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "manage_accounts")]
    ManageAccounts,
    #[serde(rename = "manage_search")]
    ManageSearch,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "maps_home_work")]
    MapsHomeWork,
    #[serde(rename = "maps_ugc")]
    MapsUgc,
    #[serde(rename = "margin")]
    Margin,
    #[serde(rename = "mark_as_unread")]
    MarkAsUnread,
    #[serde(rename = "mark_chat_read")]
    MarkChatRead,
    #[serde(rename = "mark_chat_unread")]
    MarkChatUnread,
    #[serde(rename = "mark_email_read")]
    MarkEmailRead,
    #[serde(rename = "mark_email_unread")]
    MarkEmailUnread,
    #[serde(rename = "markunread")]
    Markunread,
    #[serde(rename = "markunread_mailbox")]
    MarkunreadMailbox,
    #[serde(rename = "masks")]
    Masks,
    #[serde(rename = "maximize")]
    Maximize,
    #[serde(rename = "media_bluetooth_off")]
    MediaBluetoothOff,
    #[serde(rename = "media_bluetooth_on")]
    MediaBluetoothOn,
    #[serde(rename = "mediation")]
    Mediation,
    #[serde(rename = "medical_services")]
    MedicalServices,
    #[serde(rename = "medication")]
    Medication,
    #[serde(rename = "meeting_room")]
    MeetingRoom,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "menu")]
    Menu,
    #[serde(rename = "menu_book")]
    MenuBook,
    #[serde(rename = "menu_open")]
    MenuOpen,
    #[serde(rename = "merge_type")]
    MergeType,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "messenger")]
    Messenger,
    #[serde(rename = "messenger_outline")]
    MessengerOutline,
    #[serde(rename = "mic")]
    Mic,
    #[serde(rename = "mic_external_off")]
    MicExternalOff,
    #[serde(rename = "mic_external_on")]
    MicExternalOn,
    #[serde(rename = "mic_none")]
    MicNone,
    #[serde(rename = "mic_off")]
    MicOff,
    #[serde(rename = "microwave")]
    Microwave,
    #[serde(rename = "military_tech")]
    MilitaryTech,
    #[serde(rename = "minimize")]
    Minimize,
    #[serde(rename = "miscellaneous_services")]
    MiscellaneousServices,
    #[serde(rename = "missed_video_call")]
    MissedVideoCall,
    #[serde(rename = "mms")]
    Mms,
    #[serde(rename = "mobile_friendly")]
    MobileFriendly,
    #[serde(rename = "mobile_off")]
    MobileOff,
    #[serde(rename = "mobile_screen_share")]
    MobileScreenShare,
    #[serde(rename = "mobiledata_off")]
    MobiledataOff,
    #[serde(rename = "mode")]
    Mode,
    #[serde(rename = "mode_comment")]
    ModeComment,
    #[serde(rename = "mode_edit")]
    ModeEdit,
    #[serde(rename = "mode_edit_outline")]
    ModeEditOutline,
    #[serde(rename = "mode_night")]
    ModeNight,
    #[serde(rename = "mode_standby")]
    ModeStandby,
    #[serde(rename = "model_training")]
    ModelTraining,
    #[serde(rename = "monetization_on")]
    MonetizationOn,
    #[serde(rename = "money")]
    Money,
    #[serde(rename = "money_off")]
    MoneyOff,
    #[serde(rename = "money_off_csred")]
    MoneyOffCsred,
    #[serde(rename = "monitor")]
    Monitor,
    #[serde(rename = "monitor_weight")]
    MonitorWeight,
    #[serde(rename = "monochrome_photos")]
    MonochromePhotos,
    #[serde(rename = "mood")]
    Mood,
    #[serde(rename = "mood_bad")]
    MoodBad,
    #[serde(rename = "moped")]
    Moped,
    #[serde(rename = "more")]
    More,
    #[serde(rename = "more_horiz")]
    MoreHoriz,
    #[serde(rename = "more_time")]
    MoreTime,
    #[serde(rename = "more_vert")]
    MoreVert,
    #[serde(rename = "motion_photos_auto")]
    MotionPhotosAuto,
    #[serde(rename = "motion_photos_off")]
    MotionPhotosOff,
    #[serde(rename = "motion_photos_on")]
    MotionPhotosOn,
    #[serde(rename = "motion_photos_pause")]
    MotionPhotosPause,
    #[serde(rename = "motion_photos_paused")]
    MotionPhotosPaused,
    #[serde(rename = "motorcycle")]
    Motorcycle,
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "move_to_inbox")]
    MoveToInbox,
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "movie_creation")]
    MovieCreation,
    #[serde(rename = "movie_filter")]
    MovieFilter,
    #[serde(rename = "moving")]
    Moving,
    #[serde(rename = "mp")]
    Mp,
    #[serde(rename = "multiline_chart")]
    MultilineChart,
    #[serde(rename = "multiple_stop")]
    MultipleStop,
    #[serde(rename = "multitrack_audio")]
    MultitrackAudio,
    #[serde(rename = "museum")]
    Museum,
    #[serde(rename = "music_note")]
    MusicNote,
    #[serde(rename = "music_off")]
    MusicOff,
    #[serde(rename = "music_video")]
    MusicVideo,
    #[serde(rename = "my_library_add")]
    MyLibraryAdd,
    #[serde(rename = "my_library_books")]
    MyLibraryBooks,
    #[serde(rename = "my_library_music")]
    MyLibraryMusic,
    #[serde(rename = "my_location")]
    MyLocation,
    #[serde(rename = "nat")]
    Nat,
    #[serde(rename = "nature")]
    Nature,
    #[serde(rename = "nature_people")]
    NaturePeople,
    #[serde(rename = "navigate_before")]
    NavigateBefore,
    #[serde(rename = "navigate_next")]
    NavigateNext,
    #[serde(rename = "navigation")]
    Navigation,
    #[serde(rename = "near_me")]
    NearMe,
    #[serde(rename = "near_me_disabled")]
    NearMeDisabled,
    #[serde(rename = "nearby_error")]
    NearbyError,
    #[serde(rename = "nearby_off")]
    NearbyOff,
    #[serde(rename = "network_cell")]
    NetworkCell,
    #[serde(rename = "network_check")]
    NetworkCheck,
    #[serde(rename = "network_locked")]
    NetworkLocked,
    #[serde(rename = "network_wifi")]
    NetworkWifi,
    #[serde(rename = "new_label")]
    NewLabel,
    #[serde(rename = "new_releases")]
    NewReleases,
    #[serde(rename = "next_plan")]
    NextPlan,
    #[serde(rename = "next_week")]
    NextWeek,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "night_shelter")]
    NightShelter,
    #[serde(rename = "nightlife")]
    Nightlife,
    #[serde(rename = "nightlight")]
    Nightlight,
    #[serde(rename = "nightlight_round")]
    NightlightRound,
    #[serde(rename = "nights_stay")]
    NightsStay,
    #[serde(rename = "nine_k")]
    NineK,
    #[serde(rename = "nine_k_plus")]
    NineKPlus,
    #[serde(rename = "nine_mp")]
    NineMp,
    #[serde(rename = "nineteen_mp")]
    NineteenMp,
    #[serde(rename = "no_accounts")]
    NoAccounts,
    #[serde(rename = "no_backpack")]
    NoBackpack,
    #[serde(rename = "no_cell")]
    NoCell,
    #[serde(rename = "no_drinks")]
    NoDrinks,
    #[serde(rename = "no_encryption")]
    NoEncryption,
    #[serde(rename = "no_encryption_gmailerrorred")]
    NoEncryptionGmailerrorred,
    #[serde(rename = "no_flash")]
    NoFlash,
    #[serde(rename = "no_food")]
    NoFood,
    #[serde(rename = "no_luggage")]
    NoLuggage,
    #[serde(rename = "no_meals")]
    NoMeals,
    #[serde(rename = "no_meals_ouline")]
    NoMealsOuline,
    #[serde(rename = "no_meeting_room")]
    NoMeetingRoom,
    #[serde(rename = "no_photography")]
    NoPhotography,
    #[serde(rename = "no_sim")]
    NoSim,
    #[serde(rename = "no_stroller")]
    NoStroller,
    #[serde(rename = "no_transfer")]
    NoTransfer,
    #[serde(rename = "nordic_walking")]
    NordicWalking,
    #[serde(rename = "north")]
    North,
    #[serde(rename = "north_east")]
    NorthEast,
    #[serde(rename = "north_west")]
    NorthWest,
    #[serde(rename = "not_accessible")]
    NotAccessible,
    #[serde(rename = "not_interested")]
    NotInterested,
    #[serde(rename = "not_listed_location")]
    NotListedLocation,
    #[serde(rename = "not_started")]
    NotStarted,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "note_add")]
    NoteAdd,
    #[serde(rename = "note_alt")]
    NoteAlt,
    #[serde(rename = "notes")]
    Notes,
    #[serde(rename = "notification_add")]
    NotificationAdd,
    #[serde(rename = "notification_important")]
    NotificationImportant,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "notifications_active")]
    NotificationsActive,
    #[serde(rename = "notifications_none")]
    NotificationsNone,
    #[serde(rename = "notifications_off")]
    NotificationsOff,
    #[serde(rename = "notifications_on")]
    NotificationsOn,
    #[serde(rename = "notifications_paused")]
    NotificationsPaused,
    #[serde(rename = "now_wallpaper")]
    NowWallpaper,
    #[serde(rename = "now_widgets")]
    NowWidgets,
    #[serde(rename = "offline_bolt")]
    OfflineBolt,
    #[serde(rename = "offline_pin")]
    OfflinePin,
    #[serde(rename = "offline_share")]
    OfflineShare,
    #[serde(rename = "ondemand_video")]
    OndemandVideo,
    #[serde(rename = "one_k")]
    OneK,
    #[serde(rename = "one_k_plus")]
    OneKPlus,
    #[serde(rename = "one_x_mobiledata")]
    OneXMobiledata,
    #[serde(rename = "online_prediction")]
    OnlinePrediction,
    #[serde(rename = "opacity")]
    Opacity,
    #[serde(rename = "open_in_browser")]
    OpenInBrowser,
    #[serde(rename = "open_in_full")]
    OpenInFull,
    #[serde(rename = "open_in_new")]
    OpenInNew,
    #[serde(rename = "open_in_new_off")]
    OpenInNewOff,
    #[serde(rename = "open_with")]
    OpenWith,
    #[serde(rename = "other_houses")]
    OtherHouses,
    #[serde(rename = "outbond")]
    Outbond,
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "outbox")]
    Outbox,
    #[serde(rename = "outdoor_grill")]
    OutdoorGrill,
    #[serde(rename = "outgoing_mail")]
    OutgoingMail,
    #[serde(rename = "outlet")]
    Outlet,
    #[serde(rename = "outlined_flag")]
    OutlinedFlag,
    #[serde(rename = "padding")]
    Padding,
    #[serde(rename = "pages")]
    Pages,
    #[serde(rename = "pageview")]
    Pageview,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "palette")]
    Palette,
    #[serde(rename = "pan_tool")]
    PanTool,
    #[serde(rename = "panorama")]
    Panorama,
    #[serde(rename = "panorama_fish_eye")]
    PanoramaFishEye,
    #[serde(rename = "panorama_fisheye")]
    PanoramaFisheye,
    #[serde(rename = "panorama_horizontal")]
    PanoramaHorizontal,
    #[serde(rename = "panorama_horizontal_select")]
    PanoramaHorizontalSelect,
    #[serde(rename = "panorama_photosphere")]
    PanoramaPhotosphere,
    #[serde(rename = "panorama_photosphere_select")]
    PanoramaPhotosphereSelect,
    #[serde(rename = "panorama_vertical")]
    PanoramaVertical,
    #[serde(rename = "panorama_vertical_select")]
    PanoramaVerticalSelect,
    #[serde(rename = "panorama_wide_angle")]
    PanoramaWideAngle,
    #[serde(rename = "panorama_wide_angle_select")]
    PanoramaWideAngleSelect,
    #[serde(rename = "paragliding")]
    Paragliding,
    #[serde(rename = "park")]
    Park,
    #[serde(rename = "party_mode")]
    PartyMode,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "pattern")]
    Pattern,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "pause_circle")]
    PauseCircle,
    #[serde(rename = "pause_circle_filled")]
    PauseCircleFilled,
    #[serde(rename = "pause_circle_outline")]
    PauseCircleOutline,
    #[serde(rename = "pause_presentation")]
    PausePresentation,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "payments")]
    Payments,
    #[serde(rename = "pedal_bike")]
    PedalBike,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "pending_actions")]
    PendingActions,
    #[serde(rename = "people")]
    People,
    #[serde(rename = "people_alt")]
    PeopleAlt,
    #[serde(rename = "people_outline")]
    PeopleOutline,
    #[serde(rename = "perm_camera_mic")]
    PermCameraMic,
    #[serde(rename = "perm_contact_cal")]
    PermContactCal,
    #[serde(rename = "perm_contact_calendar")]
    PermContactCalendar,
    #[serde(rename = "perm_data_setting")]
    PermDataSetting,
    #[serde(rename = "perm_device_info")]
    PermDeviceInfo,
    #[serde(rename = "perm_device_information")]
    PermDeviceInformation,
    #[serde(rename = "perm_identity")]
    PermIdentity,
    #[serde(rename = "perm_media")]
    PermMedia,
    #[serde(rename = "perm_phone_msg")]
    PermPhoneMsg,
    #[serde(rename = "perm_scan_wifi")]
    PermScanWifi,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "person_add")]
    PersonAdd,
    #[serde(rename = "person_add_alt")]
    PersonAddAlt,
    #[serde(rename = "person_add_alt_1")]
    PersonAddAlt1,
    #[serde(rename = "person_add_disabled")]
    PersonAddDisabled,
    #[serde(rename = "person_off")]
    PersonOff,
    #[serde(rename = "person_outline")]
    PersonOutline,
    #[serde(rename = "person_pin")]
    PersonPin,
    #[serde(rename = "person_pin_circle")]
    PersonPinCircle,
    #[serde(rename = "person_remove")]
    PersonRemove,
    #[serde(rename = "person_remove_alt_1")]
    PersonRemoveAlt1,
    #[serde(rename = "person_search")]
    PersonSearch,
    #[serde(rename = "personal_injury")]
    PersonalInjury,
    #[serde(rename = "personal_video")]
    PersonalVideo,
    #[serde(rename = "pest_control")]
    PestControl,
    #[serde(rename = "pest_control_rodent")]
    PestControlRodent,
    #[serde(rename = "pets")]
    Pets,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "phone_android")]
    PhoneAndroid,
    #[serde(rename = "phone_bluetooth_speaker")]
    PhoneBluetoothSpeaker,
    #[serde(rename = "phone_callback")]
    PhoneCallback,
    #[serde(rename = "phone_disabled")]
    PhoneDisabled,
    #[serde(rename = "phone_enabled")]
    PhoneEnabled,
    #[serde(rename = "phone_forwarded")]
    PhoneForwarded,
    #[serde(rename = "phone_in_talk")]
    PhoneInTalk,
    #[serde(rename = "phone_iphone")]
    PhoneIphone,
    #[serde(rename = "phone_locked")]
    PhoneLocked,
    #[serde(rename = "phone_missed")]
    PhoneMissed,
    #[serde(rename = "phone_paused")]
    PhonePaused,
    #[serde(rename = "phonelink")]
    Phonelink,
    #[serde(rename = "phonelink_erase")]
    PhonelinkErase,
    #[serde(rename = "phonelink_lock")]
    PhonelinkLock,
    #[serde(rename = "phonelink_off")]
    PhonelinkOff,
    #[serde(rename = "phonelink_ring")]
    PhonelinkRing,
    #[serde(rename = "phonelink_setup")]
    PhonelinkSetup,
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "photo_album")]
    PhotoAlbum,
    #[serde(rename = "photo_camera")]
    PhotoCamera,
    #[serde(rename = "photo_camera_back")]
    PhotoCameraBack,
    #[serde(rename = "photo_camera_front")]
    PhotoCameraFront,
    #[serde(rename = "photo_filter")]
    PhotoFilter,
    #[serde(rename = "photo_library")]
    PhotoLibrary,
    #[serde(rename = "photo_size_select_actual")]
    PhotoSizeSelectActual,
    #[serde(rename = "photo_size_select_large")]
    PhotoSizeSelectLarge,
    #[serde(rename = "photo_size_select_small")]
    PhotoSizeSelectSmall,
    #[serde(rename = "piano")]
    Piano,
    #[serde(rename = "piano_off")]
    PianoOff,
    #[serde(rename = "picture_as_pdf")]
    PictureAsPdf,
    #[serde(rename = "picture_in_picture")]
    PictureInPicture,
    #[serde(rename = "picture_in_picture_alt")]
    PictureInPictureAlt,
    #[serde(rename = "pie_chart")]
    PieChart,
    #[serde(rename = "pie_chart_outline")]
    PieChartOutline,
    #[serde(rename = "pin")]
    Pin,
    #[serde(rename = "pin_drop")]
    PinDrop,
    #[serde(rename = "pivot_table_chart")]
    PivotTableChart,
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "plagiarism")]
    Plagiarism,
    #[serde(rename = "play_arrow")]
    PlayArrow,
    #[serde(rename = "play_circle")]
    PlayCircle,
    #[serde(rename = "play_circle_fill")]
    PlayCircleFill,
    #[serde(rename = "play_circle_filled")]
    PlayCircleFilled,
    #[serde(rename = "play_circle_outline")]
    PlayCircleOutline,
    #[serde(rename = "play_disabled")]
    PlayDisabled,
    #[serde(rename = "play_for_work")]
    PlayForWork,
    #[serde(rename = "play_lesson")]
    PlayLesson,
    #[serde(rename = "playlist_add")]
    PlaylistAdd,
    #[serde(rename = "playlist_add_check")]
    PlaylistAddCheck,
    #[serde(rename = "playlist_play")]
    PlaylistPlay,
    #[serde(rename = "plumbing")]
    Plumbing,
    #[serde(rename = "plus_one")]
    PlusOne,
    #[serde(rename = "podcasts")]
    Podcasts,
    #[serde(rename = "point_of_sale")]
    PointOfSale,
    #[serde(rename = "policy")]
    Policy,
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "polymer")]
    Polymer,
    #[serde(rename = "pool")]
    Pool,
    #[serde(rename = "portable_wifi_off")]
    PortableWifiOff,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "post_add")]
    PostAdd,
    #[serde(rename = "power")]
    Power,
    #[serde(rename = "power_input")]
    PowerInput,
    #[serde(rename = "power_off")]
    PowerOff,
    #[serde(rename = "power_settings_new")]
    PowerSettingsNew,
    #[serde(rename = "precision_manufacturing")]
    PrecisionManufacturing,
    #[serde(rename = "pregnant_woman")]
    PregnantWoman,
    #[serde(rename = "present_to_all")]
    PresentToAll,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "price_change")]
    PriceChange,
    #[serde(rename = "price_check")]
    PriceCheck,
    #[serde(rename = "print")]
    Print,
    #[serde(rename = "print_disabled")]
    PrintDisabled,
    #[serde(rename = "priority_high")]
    PriorityHigh,
    #[serde(rename = "privacy_tip")]
    PrivacyTip,
    #[serde(rename = "production_quantity_limits")]
    ProductionQuantityLimits,
    #[serde(rename = "psychology")]
    Psychology,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "public_off")]
    PublicOff,
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "published_with_changes")]
    PublishedWithChanges,
    #[serde(rename = "push_pin")]
    PushPin,
    #[serde(rename = "qr_code")]
    QrCode,
    #[serde(rename = "qr_code_2")]
    QrCode2,
    #[serde(rename = "qr_code_scanner")]
    QrCodeScanner,
    #[serde(rename = "query_builder")]
    QueryBuilder,
    #[serde(rename = "query_stats")]
    QueryStats,
    #[serde(rename = "question_answer")]
    QuestionAnswer,
    #[serde(rename = "queue")]
    Queue,
    #[serde(rename = "queue_music")]
    QueueMusic,
    #[serde(rename = "queue_play_next")]
    QueuePlayNext,
    #[serde(rename = "quick_contacts_dialer")]
    QuickContactsDialer,
    #[serde(rename = "quick_contacts_mail")]
    QuickContactsMail,
    #[serde(rename = "quickreply")]
    Quickreply,
    #[serde(rename = "quiz")]
    Quiz,
    #[serde(rename = "r_mobiledata")]
    RMobiledata,
    #[serde(rename = "radar")]
    Radar,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "radio_button_checked")]
    RadioButtonChecked,
    #[serde(rename = "radio_button_off")]
    RadioButtonOff,
    #[serde(rename = "radio_button_on")]
    RadioButtonOn,
    #[serde(rename = "radio_button_unchecked")]
    RadioButtonUnchecked,
    #[serde(rename = "railway_alert")]
    RailwayAlert,
    #[serde(rename = "ramen_dining")]
    RamenDining,
    #[serde(rename = "rate_review")]
    RateReview,
    #[serde(rename = "raw_off")]
    RawOff,
    #[serde(rename = "raw_on")]
    RawOn,
    #[serde(rename = "read_more")]
    ReadMore,
    #[serde(rename = "real_estate_agent")]
    RealEstateAgent,
    #[serde(rename = "receipt")]
    Receipt,
    #[serde(rename = "receipt_long")]
    ReceiptLong,
    #[serde(rename = "recent_actors")]
    RecentActors,
    #[serde(rename = "recommend")]
    Recommend,
    #[serde(rename = "record_voice_over")]
    RecordVoiceOver,
    #[serde(rename = "redeem")]
    Redeem,
    #[serde(rename = "redo")]
    Redo,
    #[serde(rename = "reduce_capacity")]
    ReduceCapacity,
    #[serde(rename = "refresh")]
    Refresh,
    #[serde(rename = "remember_me")]
    RememberMe,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "remove_circle")]
    RemoveCircle,
    #[serde(rename = "remove_circle_outline")]
    RemoveCircleOutline,
    #[serde(rename = "remove_done")]
    RemoveDone,
    #[serde(rename = "remove_from_queue")]
    RemoveFromQueue,
    #[serde(rename = "remove_moderator")]
    RemoveModerator,
    #[serde(rename = "remove_red_eye")]
    RemoveRedEye,
    #[serde(rename = "remove_shopping_cart")]
    RemoveShoppingCart,
    #[serde(rename = "reorder")]
    Reorder,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "repeat_on")]
    RepeatOn,
    #[serde(rename = "repeat_one")]
    RepeatOne,
    #[serde(rename = "repeat_one_on")]
    RepeatOneOn,
    #[serde(rename = "replay")]
    Replay,
    #[serde(rename = "replay_10")]
    Replay10,
    #[serde(rename = "replay_30")]
    Replay30,
    #[serde(rename = "replay_5")]
    Replay5,
    #[serde(rename = "replay_circle_filled")]
    ReplayCircleFilled,
    #[serde(rename = "reply")]
    Reply,
    #[serde(rename = "reply_all")]
    ReplyAll,
    #[serde(rename = "report")]
    Report,
    #[serde(rename = "report_gmailerrorred")]
    ReportGmailerrorred,
    #[serde(rename = "report_off")]
    ReportOff,
    #[serde(rename = "report_problem")]
    ReportProblem,
    #[serde(rename = "request_page")]
    RequestPage,
    #[serde(rename = "request_quote")]
    RequestQuote,
    #[serde(rename = "reset_tv")]
    ResetTv,
    #[serde(rename = "restart_alt")]
    RestartAlt,
    #[serde(rename = "restaurant")]
    Restaurant,
    #[serde(rename = "restaurant_menu")]
    RestaurantMenu,
    #[serde(rename = "restore")]
    Restore,
    #[serde(rename = "restore_from_trash")]
    RestoreFromTrash,
    #[serde(rename = "restore_page")]
    RestorePage,
    #[serde(rename = "reviews")]
    Reviews,
    #[serde(rename = "rice_bowl")]
    RiceBowl,
    #[serde(rename = "ring_volume")]
    RingVolume,
    #[serde(rename = "roofing")]
    Roofing,
    #[serde(rename = "room")]
    Room,
    #[serde(rename = "room_preferences")]
    RoomPreferences,
    #[serde(rename = "room_service")]
    RoomService,
    #[serde(rename = "rotate_90_degrees_ccw")]
    Rotate90DegreesCcw,
    #[serde(rename = "rotate_left")]
    RotateLeft,
    #[serde(rename = "rotate_right")]
    RotateRight,
    #[serde(rename = "rounded_corner")]
    RoundedCorner,
    #[serde(rename = "router")]
    Router,
    #[serde(rename = "rowing")]
    Rowing,
    #[serde(rename = "rss_feed")]
    RssFeed,
    #[serde(rename = "rsvp")]
    Rsvp,
    #[serde(rename = "rtt")]
    Rtt,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "rule_folder")]
    RuleFolder,
    #[serde(rename = "run_circle")]
    RunCircle,
    #[serde(rename = "running_with_errors")]
    RunningWithErrors,
    #[serde(rename = "rv_hookup")]
    RvHookup,
    #[serde(rename = "safety_divider")]
    SafetyDivider,
    #[serde(rename = "sailing")]
    Sailing,
    #[serde(rename = "sanitizer")]
    Sanitizer,
    #[serde(rename = "satellite")]
    Satellite,
    #[serde(rename = "save")]
    Save,
    #[serde(rename = "save_alt")]
    SaveAlt,
    #[serde(rename = "saved_search")]
    SavedSearch,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "scanner")]
    Scanner,
    #[serde(rename = "scatter_plot")]
    ScatterPlot,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "schedule_send")]
    ScheduleSend,
    #[serde(rename = "schema")]
    Schema,
    #[serde(rename = "school")]
    School,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "score")]
    Score,
    #[serde(rename = "screen_lock_landscape")]
    ScreenLockLandscape,
    #[serde(rename = "screen_lock_portrait")]
    ScreenLockPortrait,
    #[serde(rename = "screen_lock_rotation")]
    ScreenLockRotation,
    #[serde(rename = "screen_rotation")]
    ScreenRotation,
    #[serde(rename = "screen_search_desktop")]
    ScreenSearchDesktop,
    #[serde(rename = "screen_share")]
    ScreenShare,
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "sd_card")]
    SdCard,
    #[serde(rename = "sd_card_alert")]
    SdCardAlert,
    #[serde(rename = "sd_storage")]
    SdStorage,
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "search_off")]
    SearchOff,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "security_update")]
    SecurityUpdate,
    #[serde(rename = "security_update_good")]
    SecurityUpdateGood,
    #[serde(rename = "security_update_warning")]
    SecurityUpdateWarning,
    #[serde(rename = "segment")]
    Segment,
    #[serde(rename = "select_all")]
    SelectAll,
    #[serde(rename = "self_improvement")]
    SelfImprovement,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "send_and_archive")]
    SendAndArchive,
    #[serde(rename = "send_to_mobile")]
    SendToMobile,
    #[serde(rename = "sensor_door")]
    SensorDoor,
    #[serde(rename = "sensor_window")]
    SensorWindow,
    #[serde(rename = "sensors")]
    Sensors,
    #[serde(rename = "sensors_off")]
    SensorsOff,
    #[serde(rename = "sentiment_dissatisfied")]
    SentimentDissatisfied,
    #[serde(rename = "sentiment_neutral")]
    SentimentNeutral,
    #[serde(rename = "sentiment_satisfied")]
    SentimentSatisfied,
    #[serde(rename = "sentiment_satisfied_alt")]
    SentimentSatisfiedAlt,
    #[serde(rename = "sentiment_very_dissatisfied")]
    SentimentVeryDissatisfied,
    #[serde(rename = "sentiment_very_satisfied")]
    SentimentVerySatisfied,
    #[serde(rename = "set_meal")]
    SetMeal,
    #[serde(rename = "settings")]
    Settings,
    #[serde(rename = "settings_accessibility")]
    SettingsAccessibility,
    #[serde(rename = "settings_applications")]
    SettingsApplications,
    #[serde(rename = "settings_backup_restore")]
    SettingsBackupRestore,
    #[serde(rename = "settings_bluetooth")]
    SettingsBluetooth,
    #[serde(rename = "settings_brightness")]
    SettingsBrightness,
    #[serde(rename = "settings_cell")]
    SettingsCell,
    #[serde(rename = "settings_display")]
    SettingsDisplay,
    #[serde(rename = "settings_ethernet")]
    SettingsEthernet,
    #[serde(rename = "settings_input_antenna")]
    SettingsInputAntenna,
    #[serde(rename = "settings_input_component")]
    SettingsInputComponent,
    #[serde(rename = "settings_input_composite")]
    SettingsInputComposite,
    #[serde(rename = "settings_input_hdmi")]
    SettingsInputHdmi,
    #[serde(rename = "settings_input_svideo")]
    SettingsInputSvideo,
    #[serde(rename = "settings_overscan")]
    SettingsOverscan,
    #[serde(rename = "settings_phone")]
    SettingsPhone,
    #[serde(rename = "settings_power")]
    SettingsPower,
    #[serde(rename = "settings_remote")]
    SettingsRemote,
    #[serde(rename = "settings_suggest")]
    SettingsSuggest,
    #[serde(rename = "settings_system_daydream")]
    SettingsSystemDaydream,
    #[serde(rename = "settings_voice")]
    SettingsVoice,
    #[serde(rename = "seven_k")]
    SevenK,
    #[serde(rename = "seven_k_plus")]
    SevenKPlus,
    #[serde(rename = "seven_mp")]
    SevenMp,
    #[serde(rename = "seventeen_mp")]
    SeventeenMp,
    #[serde(rename = "share")]
    Share,
    #[serde(rename = "share_arrival_time")]
    ShareArrivalTime,
    #[serde(rename = "share_location")]
    ShareLocation,
    #[serde(rename = "shield")]
    Shield,
    #[serde(rename = "shop")]
    Shop,
    #[serde(rename = "shop_2")]
    Shop2,
    #[serde(rename = "shop_two")]
    ShopTwo,
    #[serde(rename = "shopping_bag")]
    ShoppingBag,
    #[serde(rename = "shopping_basket")]
    ShoppingBasket,
    #[serde(rename = "shopping_cart")]
    ShoppingCart,
    #[serde(rename = "short_text")]
    ShortText,
    #[serde(rename = "shortcut")]
    Shortcut,
    #[serde(rename = "show_chart")]
    ShowChart,
    #[serde(rename = "shower")]
    Shower,
    #[serde(rename = "shuffle")]
    Shuffle,
    #[serde(rename = "shuffle_on")]
    ShuffleOn,
    #[serde(rename = "shutter_speed")]
    ShutterSpeed,
    #[serde(rename = "sick")]
    Sick,
    #[serde(rename = "signal_cellular_0_bar")]
    SignalCellular0Bar,
    #[serde(rename = "signal_cellular_4_bar")]
    SignalCellular4Bar,
    #[serde(rename = "signal_cellular_alt")]
    SignalCellularAlt,
    #[serde(rename = "signal_cellular_connected_no_internet_0_bar")]
    SignalCellularConnectedNoInternet0Bar,
    #[serde(rename = "signal_cellular_connected_no_internet_4_bar")]
    SignalCellularConnectedNoInternet4Bar,
    #[serde(rename = "signal_cellular_no_sim")]
    SignalCellularNoSim,
    #[serde(rename = "signal_cellular_nodata")]
    SignalCellularNodata,
    #[serde(rename = "signal_cellular_null")]
    SignalCellularNull,
    #[serde(rename = "signal_cellular_off")]
    SignalCellularOff,
    #[serde(rename = "signal_wifi_0_bar")]
    SignalWifi0Bar,
    #[serde(rename = "signal_wifi_4_bar")]
    SignalWifi4Bar,
    #[serde(rename = "signal_wifi_4_bar_lock")]
    SignalWifi4BarLock,
    #[serde(rename = "signal_wifi_bad")]
    SignalWifiBad,
    #[serde(rename = "signal_wifi_connected_no_internet_4")]
    SignalWifiConnectedNoInternet4,
    #[serde(rename = "signal_wifi_off")]
    SignalWifiOff,
    #[serde(rename = "signal_wifi_statusbar_4_bar")]
    SignalWifiStatusbar4Bar,
    #[serde(rename = "signal_wifi_statusbar_connected_no_internet_4")]
    SignalWifiStatusbarConnectedNoInternet4,
    #[serde(rename = "signal_wifi_statusbar_null")]
    SignalWifiStatusbarNull,
    #[serde(rename = "sim_card")]
    SimCard,
    #[serde(rename = "sim_card_alert")]
    SimCardAlert,
    #[serde(rename = "sim_card_download")]
    SimCardDownload,
    #[serde(rename = "single_bed")]
    SingleBed,
    #[serde(rename = "sip")]
    Sip,
    #[serde(rename = "six_ft_apart")]
    SixFtApart,
    #[serde(rename = "six_k")]
    SixK,
    #[serde(rename = "six_k_plus")]
    SixKPlus,
    #[serde(rename = "six_mp")]
    SixMp,
    #[serde(rename = "sixteen_mp")]
    SixteenMp,
    #[serde(rename = "sixty_fps")]
    SixtyFps,
    #[serde(rename = "sixty_fps_select")]
    SixtyFpsSelect,
    #[serde(rename = "skateboarding")]
    Skateboarding,
    #[serde(rename = "skip_next")]
    SkipNext,
    #[serde(rename = "skip_previous")]
    SkipPrevious,
    #[serde(rename = "sledding")]
    Sledding,
    #[serde(rename = "slideshow")]
    Slideshow,
    #[serde(rename = "slow_motion_video")]
    SlowMotionVideo,
    #[serde(rename = "smart_button")]
    SmartButton,
    #[serde(rename = "smart_display")]
    SmartDisplay,
    #[serde(rename = "smart_screen")]
    SmartScreen,
    #[serde(rename = "smart_toy")]
    SmartToy,
    #[serde(rename = "smartphone")]
    Smartphone,
    #[serde(rename = "smoke_free")]
    SmokeFree,
    #[serde(rename = "smoking_rooms")]
    SmokingRooms,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "sms_failed")]
    SmsFailed,
    #[serde(rename = "snippet_folder")]
    SnippetFolder,
    #[serde(rename = "snooze")]
    Snooze,
    #[serde(rename = "snowboarding")]
    Snowboarding,
    #[serde(rename = "snowmobile")]
    Snowmobile,
    #[serde(rename = "snowshoeing")]
    Snowshoeing,
    #[serde(rename = "soap")]
    Soap,
    #[serde(rename = "social_distance")]
    SocialDistance,
    #[serde(rename = "sort")]
    Sort,
    #[serde(rename = "sort_by_alpha")]
    SortByAlpha,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "south")]
    South,
    #[serde(rename = "south_east")]
    SouthEast,
    #[serde(rename = "south_west")]
    SouthWest,
    #[serde(rename = "spa")]
    Spa,
    #[serde(rename = "space_bar")]
    SpaceBar,
    #[serde(rename = "space_dashboard")]
    SpaceDashboard,
    #[serde(rename = "speaker")]
    Speaker,
    #[serde(rename = "speaker_group")]
    SpeakerGroup,
    #[serde(rename = "speaker_notes")]
    SpeakerNotes,
    #[serde(rename = "speaker_notes_off")]
    SpeakerNotesOff,
    #[serde(rename = "speaker_phone")]
    SpeakerPhone,
    #[serde(rename = "speed")]
    Speed,
    #[serde(rename = "spellcheck")]
    Spellcheck,
    #[serde(rename = "splitscreen")]
    Splitscreen,
    #[serde(rename = "sports")]
    Sports,
    #[serde(rename = "sports_bar")]
    SportsBar,
    #[serde(rename = "sports_baseball")]
    SportsBaseball,
    #[serde(rename = "sports_basketball")]
    SportsBasketball,
    #[serde(rename = "sports_cricket")]
    SportsCricket,
    #[serde(rename = "sports_esports")]
    SportsEsports,
    #[serde(rename = "sports_football")]
    SportsFootball,
    #[serde(rename = "sports_golf")]
    SportsGolf,
    #[serde(rename = "sports_handball")]
    SportsHandball,
    #[serde(rename = "sports_hockey")]
    SportsHockey,
    #[serde(rename = "sports_kabaddi")]
    SportsKabaddi,
    #[serde(rename = "sports_mma")]
    SportsMma,
    #[serde(rename = "sports_motorsports")]
    SportsMotorsports,
    #[serde(rename = "sports_rugby")]
    SportsRugby,
    #[serde(rename = "sports_score")]
    SportsScore,
    #[serde(rename = "sports_soccer")]
    SportsSoccer,
    #[serde(rename = "sports_tennis")]
    SportsTennis,
    #[serde(rename = "sports_volleyball")]
    SportsVolleyball,
    #[serde(rename = "square_foot")]
    SquareFoot,
    #[serde(rename = "stacked_bar_chart")]
    StackedBarChart,
    #[serde(rename = "stacked_line_chart")]
    StackedLineChart,
    #[serde(rename = "stairs")]
    Stairs,
    #[serde(rename = "star")]
    Star,
    #[serde(rename = "star_border")]
    StarBorder,
    #[serde(rename = "star_border_purple500")]
    StarBorderPurple500,
    #[serde(rename = "star_half")]
    StarHalf,
    #[serde(rename = "star_outline")]
    StarOutline,
    #[serde(rename = "star_purple500")]
    StarPurple500,
    #[serde(rename = "star_rate")]
    StarRate,
    #[serde(rename = "stars")]
    Stars,
    #[serde(rename = "stay_current_landscape")]
    StayCurrentLandscape,
    #[serde(rename = "stay_current_portrait")]
    StayCurrentPortrait,
    #[serde(rename = "stay_primary_landscape")]
    StayPrimaryLandscape,
    #[serde(rename = "stay_primary_portrait")]
    StayPrimaryPortrait,
    #[serde(rename = "sticky_note_2")]
    StickyNote2,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "stop_circle")]
    StopCircle,
    #[serde(rename = "stop_screen_share")]
    StopScreenShare,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "store_mall_directory")]
    StoreMallDirectory,
    #[serde(rename = "storefront")]
    Storefront,
    #[serde(rename = "storm")]
    Storm,
    #[serde(rename = "straighten")]
    Straighten,
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "streetview")]
    Streetview,
    #[serde(rename = "strikethrough_s")]
    StrikethroughS,
    #[serde(rename = "stroller")]
    Stroller,
    #[serde(rename = "style")]
    Style,
    #[serde(rename = "subdirectory_arrow_left")]
    SubdirectoryArrowLeft,
    #[serde(rename = "subdirectory_arrow_right")]
    SubdirectoryArrowRight,
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "subscript")]
    Subscript,
    #[serde(rename = "subscriptions")]
    Subscriptions,
    #[serde(rename = "subtitles")]
    Subtitles,
    #[serde(rename = "subtitles_off")]
    SubtitlesOff,
    #[serde(rename = "subway")]
    Subway,
    #[serde(rename = "summarize")]
    Summarize,
    #[serde(rename = "superscript")]
    Superscript,
    #[serde(rename = "supervised_user_circle")]
    SupervisedUserCircle,
    #[serde(rename = "supervisor_account")]
    SupervisorAccount,
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "support_agent")]
    SupportAgent,
    #[serde(rename = "surfing")]
    Surfing,
    #[serde(rename = "surround_sound")]
    SurroundSound,
    #[serde(rename = "swap_calls")]
    SwapCalls,
    #[serde(rename = "swap_horiz")]
    SwapHoriz,
    #[serde(rename = "swap_horizontal_circle")]
    SwapHorizontalCircle,
    #[serde(rename = "swap_vert")]
    SwapVert,
    #[serde(rename = "swap_vert_circle")]
    SwapVertCircle,
    #[serde(rename = "swap_vertical_circle")]
    SwapVerticalCircle,
    #[serde(rename = "swipe")]
    Swipe,
    #[serde(rename = "switch_account")]
    SwitchAccount,
    #[serde(rename = "switch_camera")]
    SwitchCamera,
    #[serde(rename = "switch_left")]
    SwitchLeft,
    #[serde(rename = "switch_right")]
    SwitchRight,
    #[serde(rename = "switch_video")]
    SwitchVideo,
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "sync_alt")]
    SyncAlt,
    #[serde(rename = "sync_disabled")]
    SyncDisabled,
    #[serde(rename = "sync_problem")]
    SyncProblem,
    #[serde(rename = "system_security_update")]
    SystemSecurityUpdate,
    #[serde(rename = "system_security_update_good")]
    SystemSecurityUpdateGood,
    #[serde(rename = "system_security_update_warning")]
    SystemSecurityUpdateWarning,
    #[serde(rename = "system_update")]
    SystemUpdate,
    #[serde(rename = "system_update_alt")]
    SystemUpdateAlt,
    #[serde(rename = "system_update_tv")]
    SystemUpdateTv,
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "tab_unselected")]
    TabUnselected,
    #[serde(rename = "table_chart")]
    TableChart,
    #[serde(rename = "table_rows")]
    TableRows,
    #[serde(rename = "table_view")]
    TableView,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "tablet_android")]
    TabletAndroid,
    #[serde(rename = "tablet_mac")]
    TabletMac,
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "tag_faces")]
    TagFaces,
    #[serde(rename = "takeout_dining")]
    TakeoutDining,
    #[serde(rename = "tap_and_play")]
    TapAndPlay,
    #[serde(rename = "tapas")]
    Tapas,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "task_alt")]
    TaskAlt,
    #[serde(rename = "taxi_alert")]
    TaxiAlert,
    #[serde(rename = "ten_k")]
    TenK,
    #[serde(rename = "ten_mp")]
    TenMp,
    #[serde(rename = "terrain")]
    Terrain,
    #[serde(rename = "text_fields")]
    TextFields,
    #[serde(rename = "text_format")]
    TextFormat,
    #[serde(rename = "text_rotate_up")]
    TextRotateUp,
    #[serde(rename = "text_rotate_vertical")]
    TextRotateVertical,
    #[serde(rename = "text_rotation_angledown")]
    TextRotationAngledown,
    #[serde(rename = "text_rotation_angleup")]
    TextRotationAngleup,
    #[serde(rename = "text_rotation_down")]
    TextRotationDown,
    #[serde(rename = "text_rotation_none")]
    TextRotationNone,
    #[serde(rename = "text_snippet")]
    TextSnippet,
    #[serde(rename = "textsms")]
    Textsms,
    #[serde(rename = "texture")]
    Texture,
    #[serde(rename = "theater_comedy")]
    TheaterComedy,
    #[serde(rename = "theaters")]
    Theaters,
    #[serde(rename = "thermostat")]
    Thermostat,
    #[serde(rename = "thermostat_auto")]
    ThermostatAuto,
    #[serde(rename = "thirteen_mp")]
    ThirteenMp,
    #[serde(rename = "thirty_fps")]
    ThirtyFps,
    #[serde(rename = "thirty_fps_select")]
    ThirtyFpsSelect,
    #[serde(rename = "three_g_mobiledata")]
    ThreeGMobiledata,
    #[serde(rename = "three_k")]
    ThreeK,
    #[serde(rename = "three_k_plus")]
    ThreeKPlus,
    #[serde(rename = "three_mp")]
    ThreeMp,
    #[serde(rename = "three_p")]
    ThreeP,
    #[serde(rename = "threed_rotation")]
    ThreedRotation,
    #[serde(rename = "threesixty")]
    Threesixty,
    #[serde(rename = "thumb_down")]
    ThumbDown,
    #[serde(rename = "thumb_down_alt")]
    ThumbDownAlt,
    #[serde(rename = "thumb_down_off_alt")]
    ThumbDownOffAlt,
    #[serde(rename = "thumb_up")]
    ThumbUp,
    #[serde(rename = "thumb_up_alt")]
    ThumbUpAlt,
    #[serde(rename = "thumb_up_off_alt")]
    ThumbUpOffAlt,
    #[serde(rename = "thumbs_up_down")]
    ThumbsUpDown,
    #[serde(rename = "time_to_leave")]
    TimeToLeave,
    #[serde(rename = "timelapse")]
    Timelapse,
    #[serde(rename = "timeline")]
    Timeline,
    #[serde(rename = "timer")]
    Timer,
    #[serde(rename = "timer_10")]
    Timer10,
    #[serde(rename = "timer_10_select")]
    Timer10Select,
    #[serde(rename = "timer_3")]
    Timer3,
    #[serde(rename = "timer_3_select")]
    Timer3Select,
    #[serde(rename = "timer_off")]
    TimerOff,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "toc")]
    Toc,
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "toggle_off")]
    ToggleOff,
    #[serde(rename = "toggle_on")]
    ToggleOn,
    #[serde(rename = "toll")]
    Toll,
    #[serde(rename = "tonality")]
    Tonality,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "touch_app")]
    TouchApp,
    #[serde(rename = "tour")]
    Tour,
    #[serde(rename = "toys")]
    Toys,
    #[serde(rename = "track_changes")]
    TrackChanges,
    #[serde(rename = "traffic")]
    Traffic,
    #[serde(rename = "train")]
    Train,
    #[serde(rename = "tram")]
    Tram,
    #[serde(rename = "transfer_within_a_station")]
    TransferWithinAStation,
    #[serde(rename = "transform")]
    Transform,
    #[serde(rename = "transgender")]
    Transgender,
    #[serde(rename = "transit_enterexit")]
    TransitEnterexit,
    #[serde(rename = "translate")]
    Translate,
    #[serde(rename = "travel_explore")]
    TravelExplore,
    #[serde(rename = "trending_down")]
    TrendingDown,
    #[serde(rename = "trending_flat")]
    TrendingFlat,
    #[serde(rename = "trending_neutral")]
    TrendingNeutral,
    #[serde(rename = "trending_up")]
    TrendingUp,
    #[serde(rename = "trip_origin")]
    TripOrigin,
    #[serde(rename = "try_sms_star")]
    TrySmsStar,
    #[serde(rename = "tty")]
    Tty,
    #[serde(rename = "tune")]
    Tune,
    #[serde(rename = "tungsten")]
    Tungsten,
    #[serde(rename = "turned_in")]
    TurnedIn,
    #[serde(rename = "turned_in_not")]
    TurnedInNot,
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "tv_off")]
    TvOff,
    #[serde(rename = "twelve_mp")]
    TwelveMp,
    #[serde(rename = "twenty_four_mp")]
    TwentyFourMp,
    #[serde(rename = "twenty_mp")]
    TwentyMp,
    #[serde(rename = "twenty_one_mp")]
    TwentyOneMp,
    #[serde(rename = "twenty_three_mp")]
    TwentyThreeMp,
    #[serde(rename = "twenty_two_mp")]
    TwentyTwoMp,
    #[serde(rename = "two_k")]
    TwoK,
    #[serde(rename = "two_k_plus")]
    TwoKPlus,
    #[serde(rename = "two_mp")]
    TwoMp,
    #[serde(rename = "two_wheeler")]
    TwoWheeler,
    #[serde(rename = "umbrella")]
    Umbrella,
    #[serde(rename = "unarchive")]
    Unarchive,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "unfold_less")]
    UnfoldLess,
    #[serde(rename = "unfold_more")]
    UnfoldMore,
    #[serde(rename = "unpublished")]
    Unpublished,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
    #[serde(rename = "upcoming")]
    Upcoming,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "update_disabled")]
    UpdateDisabled,
    #[serde(rename = "upgrade")]
    Upgrade,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "upload_file")]
    UploadFile,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "usb_off")]
    UsbOff,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "verified_user")]
    VerifiedUser,
    #[serde(rename = "vertical_align_bottom")]
    VerticalAlignBottom,
    #[serde(rename = "vertical_align_center")]
    VerticalAlignCenter,
    #[serde(rename = "vertical_align_top")]
    VerticalAlignTop,
    #[serde(rename = "vertical_distribute")]
    VerticalDistribute,
    #[serde(rename = "vertical_split")]
    VerticalSplit,
    #[serde(rename = "vibration")]
    Vibration,
    #[serde(rename = "video_call")]
    VideoCall,
    #[serde(rename = "video_camera_back")]
    VideoCameraBack,
    #[serde(rename = "video_camera_front")]
    VideoCameraFront,
    #[serde(rename = "video_collection")]
    VideoCollection,
    #[serde(rename = "video_label")]
    VideoLabel,
    #[serde(rename = "video_library")]
    VideoLibrary,
    #[serde(rename = "video_settings")]
    VideoSettings,
    #[serde(rename = "video_stable")]
    VideoStable,
    #[serde(rename = "videocam")]
    Videocam,
    #[serde(rename = "videocam_off")]
    VideocamOff,
    #[serde(rename = "videogame_asset")]
    VideogameAsset,
    #[serde(rename = "videogame_asset_off")]
    VideogameAssetOff,
    #[serde(rename = "view_agenda")]
    ViewAgenda,
    #[serde(rename = "view_array")]
    ViewArray,
    #[serde(rename = "view_carousel")]
    ViewCarousel,
    #[serde(rename = "view_column")]
    ViewColumn,
    #[serde(rename = "view_comfortable")]
    ViewComfortable,
    #[serde(rename = "view_comfy")]
    ViewComfy,
    #[serde(rename = "view_compact")]
    ViewCompact,
    #[serde(rename = "view_day")]
    ViewDay,
    #[serde(rename = "view_headline")]
    ViewHeadline,
    #[serde(rename = "view_in_ar")]
    ViewInAr,
    #[serde(rename = "view_list")]
    ViewList,
    #[serde(rename = "view_module")]
    ViewModule,
    #[serde(rename = "view_quilt")]
    ViewQuilt,
    #[serde(rename = "view_sidebar")]
    ViewSidebar,
    #[serde(rename = "view_stream")]
    ViewStream,
    #[serde(rename = "view_week")]
    ViewWeek,
    #[serde(rename = "vignette")]
    Vignette,
    #[serde(rename = "villa")]
    Villa,
    #[serde(rename = "visibility")]
    Visibility,
    #[serde(rename = "visibility_off")]
    VisibilityOff,
    #[serde(rename = "voice_chat")]
    VoiceChat,
    #[serde(rename = "voice_over_off")]
    VoiceOverOff,
    #[serde(rename = "voicemail")]
    Voicemail,
    #[serde(rename = "volume_down")]
    VolumeDown,
    #[serde(rename = "volume_mute")]
    VolumeMute,
    #[serde(rename = "volume_off")]
    VolumeOff,
    #[serde(rename = "volume_up")]
    VolumeUp,
    #[serde(rename = "volunteer_activism")]
    VolunteerActivism,
    #[serde(rename = "vpn_key")]
    VpnKey,
    #[serde(rename = "vpn_lock")]
    VpnLock,
    #[serde(rename = "vrpano")]
    Vrpano,
    #[serde(rename = "wallet_giftcard")]
    WalletGiftcard,
    #[serde(rename = "wallet_membership")]
    WalletMembership,
    #[serde(rename = "wallet_travel")]
    WalletTravel,
    #[serde(rename = "wallpaper")]
    Wallpaper,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "warning_amber")]
    WarningAmber,
    #[serde(rename = "wash")]
    Wash,
    #[serde(rename = "watch")]
    Watch,
    #[serde(rename = "watch_later")]
    WatchLater,
    #[serde(rename = "water")]
    Water,
    #[serde(rename = "water_damage")]
    WaterDamage,
    #[serde(rename = "waterfall_chart")]
    WaterfallChart,
    #[serde(rename = "waves")]
    Waves,
    #[serde(rename = "wb_auto")]
    WbAuto,
    #[serde(rename = "wb_cloudy")]
    WbCloudy,
    #[serde(rename = "wb_incandescent")]
    WbIncandescent,
    #[serde(rename = "wb_iridescent")]
    WbIridescent,
    #[serde(rename = "wb_shade")]
    WbShade,
    #[serde(rename = "wb_sunny")]
    WbSunny,
    #[serde(rename = "wb_twighlight")]
    WbTwighlight,
    #[serde(rename = "wb_twilight")]
    WbTwilight,
    #[serde(rename = "wc")]
    Wc,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "web_asset")]
    WebAsset,
    #[serde(rename = "web_asset_off")]
    WebAssetOff,
    #[serde(rename = "web_stories")]
    WebStories,
    #[serde(rename = "weekend")]
    Weekend,
    #[serde(rename = "west")]
    West,
    #[serde(rename = "whatshot")]
    Whatshot,
    #[serde(rename = "wheelchair_pickup")]
    WheelchairPickup,
    #[serde(rename = "where_to_vote")]
    WhereToVote,
    #[serde(rename = "widgets")]
    Widgets,
    #[serde(rename = "wifi")]
    Wifi,
    #[serde(rename = "wifi_calling")]
    WifiCalling,
    #[serde(rename = "wifi_calling_3")]
    WifiCalling3,
    #[serde(rename = "wifi_lock")]
    WifiLock,
    #[serde(rename = "wifi_off")]
    WifiOff,
    #[serde(rename = "wifi_protected_setup")]
    WifiProtectedSetup,
    #[serde(rename = "wifi_tethering")]
    WifiTethering,
    #[serde(rename = "wifi_tethering_off")]
    WifiTetheringOff,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "wine_bar")]
    WineBar,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "work_off")]
    WorkOff,
    #[serde(rename = "work_outline")]
    WorkOutline,
    #[serde(rename = "workspaces")]
    Workspaces,
    #[serde(rename = "workspaces_filled")]
    WorkspacesFilled,
    #[serde(rename = "workspaces_outline")]
    WorkspacesOutline,
    #[serde(rename = "wrap_text")]
    WrapText,
    #[serde(rename = "wrong_location")]
    WrongLocation,
    #[serde(rename = "wysiwyg")]
    Wysiwyg,
    #[serde(rename = "yard")]
    Yard,
    #[serde(rename = "youtube_searched_for")]
    YoutubeSearchedFor,
    #[serde(rename = "zoom_in")]
    ZoomIn,
    #[serde(rename = "zoom_out")]
    ZoomOut,
    #[serde(rename = "zoom_out_map")]
    ZoomOutMap,
    #[serde(rename = "zoom_out_outlined")]
    ZoomOutOutlined,
}
impl From<&StylesIconName> for StylesIconName {
    fn from(value: &StylesIconName) -> Self {
        value.clone()
    }
}
impl ToString for StylesIconName {
    fn to_string(&self) -> String {
        match *self {
            Self::AcUnit => "ac_unit".to_string(),
            Self::AccessAlarm => "access_alarm".to_string(),
            Self::AccessAlarms => "access_alarms".to_string(),
            Self::AccessTime => "access_time".to_string(),
            Self::AccessTimeFilled => "access_time_filled".to_string(),
            Self::Accessibility => "accessibility".to_string(),
            Self::AccessibilityNew => "accessibility_new".to_string(),
            Self::Accessible => "accessible".to_string(),
            Self::AccessibleForward => "accessible_forward".to_string(),
            Self::AccountBalance => "account_balance".to_string(),
            Self::AccountBalanceWallet => "account_balance_wallet".to_string(),
            Self::AccountBox => "account_box".to_string(),
            Self::AccountCircle => "account_circle".to_string(),
            Self::AccountTree => "account_tree".to_string(),
            Self::AdUnits => "ad_units".to_string(),
            Self::Adb => "adb".to_string(),
            Self::Add => "add".to_string(),
            Self::AddAPhoto => "add_a_photo".to_string(),
            Self::AddAlarm => "add_alarm".to_string(),
            Self::AddAlert => "add_alert".to_string(),
            Self::AddBox => "add_box".to_string(),
            Self::AddBusiness => "add_business".to_string(),
            Self::AddCall => "add_call".to_string(),
            Self::AddChart => "add_chart".to_string(),
            Self::AddCircle => "add_circle".to_string(),
            Self::AddCircleOutline => "add_circle_outline".to_string(),
            Self::AddComment => "add_comment".to_string(),
            Self::AddIcCall => "add_ic_call".to_string(),
            Self::AddLink => "add_link".to_string(),
            Self::AddLocation => "add_location".to_string(),
            Self::AddLocationAlt => "add_location_alt".to_string(),
            Self::AddModerator => "add_moderator".to_string(),
            Self::AddPhotoAlternate => "add_photo_alternate".to_string(),
            Self::AddReaction => "add_reaction".to_string(),
            Self::AddRoad => "add_road".to_string(),
            Self::AddShoppingCart => "add_shopping_cart".to_string(),
            Self::AddTask => "add_task".to_string(),
            Self::AddToDrive => "add_to_drive".to_string(),
            Self::AddToHomeScreen => "add_to_home_screen".to_string(),
            Self::AddToPhotos => "add_to_photos".to_string(),
            Self::AddToQueue => "add_to_queue".to_string(),
            Self::Addchart => "addchart".to_string(),
            Self::Adjust => "adjust".to_string(),
            Self::AdminPanelSettings => "admin_panel_settings".to_string(),
            Self::Agriculture => "agriculture".to_string(),
            Self::Air => "air".to_string(),
            Self::AirlineSeatFlat => "airline_seat_flat".to_string(),
            Self::AirlineSeatFlatAngled => "airline_seat_flat_angled".to_string(),
            Self::AirlineSeatIndividualSuite => {
                "airline_seat_individual_suite".to_string()
            }
            Self::AirlineSeatLegroomExtra => "airline_seat_legroom_extra".to_string(),
            Self::AirlineSeatLegroomNormal => "airline_seat_legroom_normal".to_string(),
            Self::AirlineSeatLegroomReduced => "airline_seat_legroom_reduced".to_string(),
            Self::AirlineSeatReclineExtra => "airline_seat_recline_extra".to_string(),
            Self::AirlineSeatReclineNormal => "airline_seat_recline_normal".to_string(),
            Self::AirplaneTicket => "airplane_ticket".to_string(),
            Self::AirplanemodeActive => "airplanemode_active".to_string(),
            Self::AirplanemodeInactive => "airplanemode_inactive".to_string(),
            Self::AirplanemodeOff => "airplanemode_off".to_string(),
            Self::AirplanemodeOn => "airplanemode_on".to_string(),
            Self::Airplay => "airplay".to_string(),
            Self::AirportShuttle => "airport_shuttle".to_string(),
            Self::Alarm => "alarm".to_string(),
            Self::AlarmAdd => "alarm_add".to_string(),
            Self::AlarmOff => "alarm_off".to_string(),
            Self::AlarmOn => "alarm_on".to_string(),
            Self::Album => "album".to_string(),
            Self::AlignHorizontalCenter => "align_horizontal_center".to_string(),
            Self::AlignHorizontalLeft => "align_horizontal_left".to_string(),
            Self::AlignHorizontalRight => "align_horizontal_right".to_string(),
            Self::AlignVerticalBottom => "align_vertical_bottom".to_string(),
            Self::AlignVerticalCenter => "align_vertical_center".to_string(),
            Self::AlignVerticalTop => "align_vertical_top".to_string(),
            Self::AllInbox => "all_inbox".to_string(),
            Self::AllInclusive => "all_inclusive".to_string(),
            Self::AllOut => "all_out".to_string(),
            Self::AltRoute => "alt_route".to_string(),
            Self::AlternateEmail => "alternate_email".to_string(),
            Self::AmpStories => "amp_stories".to_string(),
            Self::Analytics => "analytics".to_string(),
            Self::Anchor => "anchor".to_string(),
            Self::Android => "android".to_string(),
            Self::Animation => "animation".to_string(),
            Self::Announcement => "announcement".to_string(),
            Self::Aod => "aod".to_string(),
            Self::Apartment => "apartment".to_string(),
            Self::Api => "api".to_string(),
            Self::AppBlocking => "app_blocking".to_string(),
            Self::AppRegistration => "app_registration".to_string(),
            Self::AppSettingsAlt => "app_settings_alt".to_string(),
            Self::Approval => "approval".to_string(),
            Self::Apps => "apps".to_string(),
            Self::Architecture => "architecture".to_string(),
            Self::Archive => "archive".to_string(),
            Self::ArrowBack => "arrow_back".to_string(),
            Self::ArrowBackIos => "arrow_back_ios".to_string(),
            Self::ArrowBackIosNew => "arrow_back_ios_new".to_string(),
            Self::ArrowCircleDown => "arrow_circle_down".to_string(),
            Self::ArrowCircleUp => "arrow_circle_up".to_string(),
            Self::ArrowDownward => "arrow_downward".to_string(),
            Self::ArrowDropDown => "arrow_drop_down".to_string(),
            Self::ArrowDropDownCircle => "arrow_drop_down_circle".to_string(),
            Self::ArrowDropUp => "arrow_drop_up".to_string(),
            Self::ArrowForward => "arrow_forward".to_string(),
            Self::ArrowForwardIos => "arrow_forward_ios".to_string(),
            Self::ArrowLeft => "arrow_left".to_string(),
            Self::ArrowRight => "arrow_right".to_string(),
            Self::ArrowRightAlt => "arrow_right_alt".to_string(),
            Self::ArrowUpward => "arrow_upward".to_string(),
            Self::ArtTrack => "art_track".to_string(),
            Self::Article => "article".to_string(),
            Self::AspectRatio => "aspect_ratio".to_string(),
            Self::Assessment => "assessment".to_string(),
            Self::Assignment => "assignment".to_string(),
            Self::AssignmentInd => "assignment_ind".to_string(),
            Self::AssignmentLate => "assignment_late".to_string(),
            Self::AssignmentReturn => "assignment_return".to_string(),
            Self::AssignmentReturned => "assignment_returned".to_string(),
            Self::AssignmentTurnedIn => "assignment_turned_in".to_string(),
            Self::Assistant => "assistant".to_string(),
            Self::AssistantDirection => "assistant_direction".to_string(),
            Self::AssistantNavigation => "assistant_navigation".to_string(),
            Self::AssistantPhoto => "assistant_photo".to_string(),
            Self::Atm => "atm".to_string(),
            Self::AttachEmail => "attach_email".to_string(),
            Self::AttachFile => "attach_file".to_string(),
            Self::AttachMoney => "attach_money".to_string(),
            Self::Attachment => "attachment".to_string(),
            Self::Attractions => "attractions".to_string(),
            Self::Attribution => "attribution".to_string(),
            Self::Audiotrack => "audiotrack".to_string(),
            Self::AutoAwesome => "auto_awesome".to_string(),
            Self::AutoAwesomeMosaic => "auto_awesome_mosaic".to_string(),
            Self::AutoAwesomeMotion => "auto_awesome_motion".to_string(),
            Self::AutoDelete => "auto_delete".to_string(),
            Self::AutoFixHigh => "auto_fix_high".to_string(),
            Self::AutoFixNormal => "auto_fix_normal".to_string(),
            Self::AutoFixOff => "auto_fix_off".to_string(),
            Self::AutoGraph => "auto_graph".to_string(),
            Self::AutoStories => "auto_stories".to_string(),
            Self::AutofpsSelect => "autofps_select".to_string(),
            Self::Autorenew => "autorenew".to_string(),
            Self::AvTimer => "av_timer".to_string(),
            Self::BabyChangingStation => "baby_changing_station".to_string(),
            Self::Backpack => "backpack".to_string(),
            Self::Backspace => "backspace".to_string(),
            Self::Backup => "backup".to_string(),
            Self::BackupTable => "backup_table".to_string(),
            Self::Badge => "badge".to_string(),
            Self::BakeryDining => "bakery_dining".to_string(),
            Self::Balcony => "balcony".to_string(),
            Self::Ballot => "ballot".to_string(),
            Self::BarChart => "bar_chart".to_string(),
            Self::BatchPrediction => "batch_prediction".to_string(),
            Self::Bathroom => "bathroom".to_string(),
            Self::Bathtub => "bathtub".to_string(),
            Self::BatteryAlert => "battery_alert".to_string(),
            Self::BatteryChargingFull => "battery_charging_full".to_string(),
            Self::BatteryFull => "battery_full".to_string(),
            Self::BatterySaver => "battery_saver".to_string(),
            Self::BatteryStd => "battery_std".to_string(),
            Self::BatteryUnknown => "battery_unknown".to_string(),
            Self::BeachAccess => "beach_access".to_string(),
            Self::Bed => "bed".to_string(),
            Self::BedroomBaby => "bedroom_baby".to_string(),
            Self::BedroomChild => "bedroom_child".to_string(),
            Self::BedroomParent => "bedroom_parent".to_string(),
            Self::Bedtime => "bedtime".to_string(),
            Self::Beenhere => "beenhere".to_string(),
            Self::Bento => "bento".to_string(),
            Self::BikeScooter => "bike_scooter".to_string(),
            Self::Biotech => "biotech".to_string(),
            Self::Blender => "blender".to_string(),
            Self::Block => "block".to_string(),
            Self::BlockFlipped => "block_flipped".to_string(),
            Self::Bloodtype => "bloodtype".to_string(),
            Self::Bluetooth => "bluetooth".to_string(),
            Self::BluetoothAudio => "bluetooth_audio".to_string(),
            Self::BluetoothConnected => "bluetooth_connected".to_string(),
            Self::BluetoothDisabled => "bluetooth_disabled".to_string(),
            Self::BluetoothDrive => "bluetooth_drive".to_string(),
            Self::BluetoothSearching => "bluetooth_searching".to_string(),
            Self::BlurCircular => "blur_circular".to_string(),
            Self::BlurLinear => "blur_linear".to_string(),
            Self::BlurOff => "blur_off".to_string(),
            Self::BlurOn => "blur_on".to_string(),
            Self::Bolt => "bolt".to_string(),
            Self::Book => "book".to_string(),
            Self::BookOnline => "book_online".to_string(),
            Self::Bookmark => "bookmark".to_string(),
            Self::BookmarkAdd => "bookmark_add".to_string(),
            Self::BookmarkAdded => "bookmark_added".to_string(),
            Self::BookmarkBorder => "bookmark_border".to_string(),
            Self::BookmarkOutline => "bookmark_outline".to_string(),
            Self::BookmarkRemove => "bookmark_remove".to_string(),
            Self::Bookmarks => "bookmarks".to_string(),
            Self::BorderAll => "border_all".to_string(),
            Self::BorderBottom => "border_bottom".to_string(),
            Self::BorderClear => "border_clear".to_string(),
            Self::BorderColor => "border_color".to_string(),
            Self::BorderHorizontal => "border_horizontal".to_string(),
            Self::BorderInner => "border_inner".to_string(),
            Self::BorderLeft => "border_left".to_string(),
            Self::BorderOuter => "border_outer".to_string(),
            Self::BorderRight => "border_right".to_string(),
            Self::BorderStyle => "border_style".to_string(),
            Self::BorderTop => "border_top".to_string(),
            Self::BorderVertical => "border_vertical".to_string(),
            Self::BrandingWatermark => "branding_watermark".to_string(),
            Self::BreakfastDining => "breakfast_dining".to_string(),
            Self::Brightness1 => "brightness_1".to_string(),
            Self::Brightness2 => "brightness_2".to_string(),
            Self::Brightness3 => "brightness_3".to_string(),
            Self::Brightness4 => "brightness_4".to_string(),
            Self::Brightness5 => "brightness_5".to_string(),
            Self::Brightness6 => "brightness_6".to_string(),
            Self::Brightness7 => "brightness_7".to_string(),
            Self::BrightnessAuto => "brightness_auto".to_string(),
            Self::BrightnessHigh => "brightness_high".to_string(),
            Self::BrightnessLow => "brightness_low".to_string(),
            Self::BrightnessMedium => "brightness_medium".to_string(),
            Self::BrokenImage => "broken_image".to_string(),
            Self::BrowserNotSupported => "browser_not_supported".to_string(),
            Self::BrunchDining => "brunch_dining".to_string(),
            Self::Brush => "brush".to_string(),
            Self::BubbleChart => "bubble_chart".to_string(),
            Self::BugReport => "bug_report".to_string(),
            Self::Build => "build".to_string(),
            Self::BuildCircle => "build_circle".to_string(),
            Self::Bungalow => "bungalow".to_string(),
            Self::BurstMode => "burst_mode".to_string(),
            Self::BusAlert => "bus_alert".to_string(),
            Self::Business => "business".to_string(),
            Self::BusinessCenter => "business_center".to_string(),
            Self::Cabin => "cabin".to_string(),
            Self::Cable => "cable".to_string(),
            Self::Cached => "cached".to_string(),
            Self::Cake => "cake".to_string(),
            Self::Calculate => "calculate".to_string(),
            Self::CalendarToday => "calendar_today".to_string(),
            Self::CalendarViewDay => "calendar_view_day".to_string(),
            Self::CalendarViewMonth => "calendar_view_month".to_string(),
            Self::CalendarViewWeek => "calendar_view_week".to_string(),
            Self::Call => "call".to_string(),
            Self::CallEnd => "call_end".to_string(),
            Self::CallMade => "call_made".to_string(),
            Self::CallMerge => "call_merge".to_string(),
            Self::CallMissed => "call_missed".to_string(),
            Self::CallMissedOutgoing => "call_missed_outgoing".to_string(),
            Self::CallReceived => "call_received".to_string(),
            Self::CallSplit => "call_split".to_string(),
            Self::CallToAction => "call_to_action".to_string(),
            Self::Camera => "camera".to_string(),
            Self::CameraAlt => "camera_alt".to_string(),
            Self::CameraEnhance => "camera_enhance".to_string(),
            Self::CameraFront => "camera_front".to_string(),
            Self::CameraIndoor => "camera_indoor".to_string(),
            Self::CameraOutdoor => "camera_outdoor".to_string(),
            Self::CameraRear => "camera_rear".to_string(),
            Self::CameraRoll => "camera_roll".to_string(),
            Self::Cameraswitch => "cameraswitch".to_string(),
            Self::Campaign => "campaign".to_string(),
            Self::Cancel => "cancel".to_string(),
            Self::CancelPresentation => "cancel_presentation".to_string(),
            Self::CancelScheduleSend => "cancel_schedule_send".to_string(),
            Self::CarRental => "car_rental".to_string(),
            Self::CarRepair => "car_repair".to_string(),
            Self::CardGiftcard => "card_giftcard".to_string(),
            Self::CardMembership => "card_membership".to_string(),
            Self::CardTravel => "card_travel".to_string(),
            Self::Carpenter => "carpenter".to_string(),
            Self::Cases => "cases".to_string(),
            Self::Casino => "casino".to_string(),
            Self::Cast => "cast".to_string(),
            Self::CastConnected => "cast_connected".to_string(),
            Self::CastForEducation => "cast_for_education".to_string(),
            Self::CatchingPokemon => "catching_pokemon".to_string(),
            Self::Category => "category".to_string(),
            Self::Celebration => "celebration".to_string(),
            Self::CellWifi => "cell_wifi".to_string(),
            Self::CenterFocusStrong => "center_focus_strong".to_string(),
            Self::CenterFocusWeak => "center_focus_weak".to_string(),
            Self::Chair => "chair".to_string(),
            Self::ChairAlt => "chair_alt".to_string(),
            Self::Chalet => "chalet".to_string(),
            Self::ChangeCircle => "change_circle".to_string(),
            Self::ChangeHistory => "change_history".to_string(),
            Self::ChargingStation => "charging_station".to_string(),
            Self::Chat => "chat".to_string(),
            Self::ChatBubble => "chat_bubble".to_string(),
            Self::ChatBubbleOutline => "chat_bubble_outline".to_string(),
            Self::Check => "check".to_string(),
            Self::CheckBox => "check_box".to_string(),
            Self::CheckBoxOutlineBlank => "check_box_outline_blank".to_string(),
            Self::CheckCircle => "check_circle".to_string(),
            Self::CheckCircleOutline => "check_circle_outline".to_string(),
            Self::Checklist => "checklist".to_string(),
            Self::ChecklistRtl => "checklist_rtl".to_string(),
            Self::Checkroom => "checkroom".to_string(),
            Self::ChevronLeft => "chevron_left".to_string(),
            Self::ChevronRight => "chevron_right".to_string(),
            Self::ChildCare => "child_care".to_string(),
            Self::ChildFriendly => "child_friendly".to_string(),
            Self::ChromeReaderMode => "chrome_reader_mode".to_string(),
            Self::Circle => "circle".to_string(),
            Self::CircleNotifications => "circle_notifications".to_string(),
            Self::Class => "class_".to_string(),
            Self::CleanHands => "clean_hands".to_string(),
            Self::CleaningServices => "cleaning_services".to_string(),
            Self::Clear => "clear".to_string(),
            Self::ClearAll => "clear_all".to_string(),
            Self::Close => "close".to_string(),
            Self::CloseFullscreen => "close_fullscreen".to_string(),
            Self::ClosedCaption => "closed_caption".to_string(),
            Self::ClosedCaptionDisabled => "closed_caption_disabled".to_string(),
            Self::ClosedCaptionOff => "closed_caption_off".to_string(),
            Self::Cloud => "cloud".to_string(),
            Self::CloudCircle => "cloud_circle".to_string(),
            Self::CloudDone => "cloud_done".to_string(),
            Self::CloudDownload => "cloud_download".to_string(),
            Self::CloudOff => "cloud_off".to_string(),
            Self::CloudQueue => "cloud_queue".to_string(),
            Self::CloudUpload => "cloud_upload".to_string(),
            Self::Code => "code".to_string(),
            Self::CodeOff => "code_off".to_string(),
            Self::Coffee => "coffee".to_string(),
            Self::CoffeeMaker => "coffee_maker".to_string(),
            Self::Collections => "collections".to_string(),
            Self::CollectionsBookmark => "collections_bookmark".to_string(),
            Self::ColorLens => "color_lens".to_string(),
            Self::Colorize => "colorize".to_string(),
            Self::Comment => "comment".to_string(),
            Self::CommentBank => "comment_bank".to_string(),
            Self::Commute => "commute".to_string(),
            Self::Compare => "compare".to_string(),
            Self::CompareArrows => "compare_arrows".to_string(),
            Self::CompassCalibration => "compass_calibration".to_string(),
            Self::Compress => "compress".to_string(),
            Self::Computer => "computer".to_string(),
            Self::ConfirmationNum => "confirmation_num".to_string(),
            Self::ConfirmationNumber => "confirmation_number".to_string(),
            Self::ConnectWithoutContact => "connect_without_contact".to_string(),
            Self::ConnectedTv => "connected_tv".to_string(),
            Self::Construction => "construction".to_string(),
            Self::ContactMail => "contact_mail".to_string(),
            Self::ContactPage => "contact_page".to_string(),
            Self::ContactPhone => "contact_phone".to_string(),
            Self::ContactSupport => "contact_support".to_string(),
            Self::Contactless => "contactless".to_string(),
            Self::Contacts => "contacts".to_string(),
            Self::ContentCopy => "content_copy".to_string(),
            Self::ContentCut => "content_cut".to_string(),
            Self::ContentPaste => "content_paste".to_string(),
            Self::ContentPasteOff => "content_paste_off".to_string(),
            Self::ControlCamera => "control_camera".to_string(),
            Self::ControlPoint => "control_point".to_string(),
            Self::ControlPointDuplicate => "control_point_duplicate".to_string(),
            Self::Copy => "copy".to_string(),
            Self::CopyAll => "copy_all".to_string(),
            Self::Copyright => "copyright".to_string(),
            Self::Coronavirus => "coronavirus".to_string(),
            Self::CorporateFare => "corporate_fare".to_string(),
            Self::Cottage => "cottage".to_string(),
            Self::Countertops => "countertops".to_string(),
            Self::Create => "create".to_string(),
            Self::CreateNewFolder => "create_new_folder".to_string(),
            Self::CreditCard => "credit_card".to_string(),
            Self::CreditCardOff => "credit_card_off".to_string(),
            Self::CreditScore => "credit_score".to_string(),
            Self::Crib => "crib".to_string(),
            Self::Crop => "crop".to_string(),
            Self::Crop169 => "crop_16_9".to_string(),
            Self::Crop32 => "crop_3_2".to_string(),
            Self::Crop54 => "crop_5_4".to_string(),
            Self::Crop75 => "crop_7_5".to_string(),
            Self::CropDin => "crop_din".to_string(),
            Self::CropFree => "crop_free".to_string(),
            Self::CropLandscape => "crop_landscape".to_string(),
            Self::CropOriginal => "crop_original".to_string(),
            Self::CropPortrait => "crop_portrait".to_string(),
            Self::CropRotate => "crop_rotate".to_string(),
            Self::CropSquare => "crop_square".to_string(),
            Self::Cut => "cut".to_string(),
            Self::Dangerous => "dangerous".to_string(),
            Self::DarkMode => "dark_mode".to_string(),
            Self::Dashboard => "dashboard".to_string(),
            Self::DashboardCustomize => "dashboard_customize".to_string(),
            Self::DataSaverOff => "data_saver_off".to_string(),
            Self::DataSaverOn => "data_saver_on".to_string(),
            Self::DataUsage => "data_usage".to_string(),
            Self::DateRange => "date_range".to_string(),
            Self::Deck => "deck".to_string(),
            Self::Dehaze => "dehaze".to_string(),
            Self::Delete => "delete".to_string(),
            Self::DeleteForever => "delete_forever".to_string(),
            Self::DeleteOutline => "delete_outline".to_string(),
            Self::DeleteSweep => "delete_sweep".to_string(),
            Self::DeliveryDining => "delivery_dining".to_string(),
            Self::DepartureBoard => "departure_board".to_string(),
            Self::Description => "description".to_string(),
            Self::DesignServices => "design_services".to_string(),
            Self::DesktopAccessDisabled => "desktop_access_disabled".to_string(),
            Self::DesktopMac => "desktop_mac".to_string(),
            Self::DesktopWindows => "desktop_windows".to_string(),
            Self::Details => "details".to_string(),
            Self::DeveloperBoard => "developer_board".to_string(),
            Self::DeveloperBoardOff => "developer_board_off".to_string(),
            Self::DeveloperMode => "developer_mode".to_string(),
            Self::DeviceHub => "device_hub".to_string(),
            Self::DeviceThermostat => "device_thermostat".to_string(),
            Self::DeviceUnknown => "device_unknown".to_string(),
            Self::Devices => "devices".to_string(),
            Self::DevicesOther => "devices_other".to_string(),
            Self::DialerSip => "dialer_sip".to_string(),
            Self::Dialpad => "dialpad".to_string(),
            Self::Dining => "dining".to_string(),
            Self::DinnerDining => "dinner_dining".to_string(),
            Self::Directions => "directions".to_string(),
            Self::DirectionsBike => "directions_bike".to_string(),
            Self::DirectionsBoat => "directions_boat".to_string(),
            Self::DirectionsBoatFilled => "directions_boat_filled".to_string(),
            Self::DirectionsBus => "directions_bus".to_string(),
            Self::DirectionsBusFilled => "directions_bus_filled".to_string(),
            Self::DirectionsCar => "directions_car".to_string(),
            Self::DirectionsCarFilled => "directions_car_filled".to_string(),
            Self::DirectionsFerry => "directions_ferry".to_string(),
            Self::DirectionsOff => "directions_off".to_string(),
            Self::DirectionsRailwayFilled => "directions_railway_filled".to_string(),
            Self::DirectionsRun => "directions_run".to_string(),
            Self::DirectionsRailway => "directions_railway".to_string(),
            Self::DirectionsSubway => "directions_subway".to_string(),
            Self::DirectionsSubwayFilled => "directions_subway_filled".to_string(),
            Self::DirectionsTrain => "directions_train".to_string(),
            Self::DirectionsTransit => "directions_transit".to_string(),
            Self::DirectionsTransitFilled => "directions_transit_filled".to_string(),
            Self::DirectionsWalk => "directions_walk".to_string(),
            Self::DirtyLens => "dirty_lens".to_string(),
            Self::DisabledByDefault => "disabled_by_default".to_string(),
            Self::DiscFull => "disc_full".to_string(),
            Self::DndForwardslash => "dnd_forwardslash".to_string(),
            Self::Dns => "dns".to_string(),
            Self::DoDisturb => "do_disturb".to_string(),
            Self::DoDisturbAlt => "do_disturb_alt".to_string(),
            Self::DoDisturbOff => "do_disturb_off".to_string(),
            Self::DoDisturbOn => "do_disturb_on".to_string(),
            Self::DoNotDisturb => "do_not_disturb".to_string(),
            Self::DoNotDisturbAlt => "do_not_disturb_alt".to_string(),
            Self::DoNotDisturbOff => "do_not_disturb_off".to_string(),
            Self::DoNotDisturbOn => "do_not_disturb_on".to_string(),
            Self::DoNotDisturbOnTotalSilence => {
                "do_not_disturb_on_total_silence".to_string()
            }
            Self::DoNotStep => "do_not_step".to_string(),
            Self::DoNotTouch => "do_not_touch".to_string(),
            Self::Dock => "dock".to_string(),
            Self::DocumentScanner => "document_scanner".to_string(),
            Self::Domain => "domain".to_string(),
            Self::DomainDisabled => "domain_disabled".to_string(),
            Self::DomainVerification => "domain_verification".to_string(),
            Self::Done => "done".to_string(),
            Self::DoneAll => "done_all".to_string(),
            Self::DoneOutline => "done_outline".to_string(),
            Self::DonutLarge => "donut_large".to_string(),
            Self::DonutSmall => "donut_small".to_string(),
            Self::DoorBack => "door_back".to_string(),
            Self::DoorFront => "door_front".to_string(),
            Self::DoorSliding => "door_sliding".to_string(),
            Self::Doorbell => "doorbell".to_string(),
            Self::DoubleArrow => "double_arrow".to_string(),
            Self::DownhillSkiing => "downhill_skiing".to_string(),
            Self::Download => "download".to_string(),
            Self::DownloadDone => "download_done".to_string(),
            Self::DownloadForOffline => "download_for_offline".to_string(),
            Self::Downloading => "downloading".to_string(),
            Self::Drafts => "drafts".to_string(),
            Self::DragHandle => "drag_handle".to_string(),
            Self::DragIndicator => "drag_indicator".to_string(),
            Self::DriveEta => "drive_eta".to_string(),
            Self::DriveFileMove => "drive_file_move".to_string(),
            Self::DriveFileMoveOutline => "drive_file_move_outline".to_string(),
            Self::DriveFileRenameOutline => "drive_file_rename_outline".to_string(),
            Self::DriveFolderUpload => "drive_folder_upload".to_string(),
            Self::Dry => "dry".to_string(),
            Self::DryCleaning => "dry_cleaning".to_string(),
            Self::Duo => "duo".to_string(),
            Self::Dvr => "dvr".to_string(),
            Self::DynamicFeed => "dynamic_feed".to_string(),
            Self::DynamicForm => "dynamic_form".to_string(),
            Self::EMobiledata => "e_mobiledata".to_string(),
            Self::Earbuds => "earbuds".to_string(),
            Self::EarbudsBattery => "earbuds_battery".to_string(),
            Self::East => "east".to_string(),
            Self::Eco => "eco".to_string(),
            Self::EdgesensorHigh => "edgesensor_high".to_string(),
            Self::EdgesensorLow => "edgesensor_low".to_string(),
            Self::Edit => "edit".to_string(),
            Self::EditAttributes => "edit_attributes".to_string(),
            Self::EditLocation => "edit_location".to_string(),
            Self::EditLocationAlt => "edit_location_alt".to_string(),
            Self::EditNotifications => "edit_notifications".to_string(),
            Self::EditOff => "edit_off".to_string(),
            Self::EditRoad => "edit_road".to_string(),
            Self::EightK => "eight_k".to_string(),
            Self::EightKPlus => "eight_k_plus".to_string(),
            Self::EightMp => "eight_mp".to_string(),
            Self::EighteenMp => "eighteen_mp".to_string(),
            Self::Eject => "eject".to_string(),
            Self::Elderly => "elderly".to_string(),
            Self::ElectricBike => "electric_bike".to_string(),
            Self::ElectricCar => "electric_car".to_string(),
            Self::ElectricMoped => "electric_moped".to_string(),
            Self::ElectricRickshaw => "electric_rickshaw".to_string(),
            Self::ElectricScooter => "electric_scooter".to_string(),
            Self::ElectricalServices => "electrical_services".to_string(),
            Self::Elevator => "elevator".to_string(),
            Self::ElevenMp => "eleven_mp".to_string(),
            Self::Email => "email".to_string(),
            Self::EmojiEmotions => "emoji_emotions".to_string(),
            Self::EmojiEvents => "emoji_events".to_string(),
            Self::EmojiFlags => "emoji_flags".to_string(),
            Self::EmojiFoodBeverage => "emoji_food_beverage".to_string(),
            Self::EmojiNature => "emoji_nature".to_string(),
            Self::EmojiObjects => "emoji_objects".to_string(),
            Self::EmojiPeople => "emoji_people".to_string(),
            Self::EmojiSymbols => "emoji_symbols".to_string(),
            Self::EmojiTransportation => "emoji_transportation".to_string(),
            Self::Engineering => "engineering".to_string(),
            Self::EnhancePhotoTranslate => "enhance_photo_translate".to_string(),
            Self::EnhancedEncryption => "enhanced_encryption".to_string(),
            Self::Equalizer => "equalizer".to_string(),
            Self::Error => "error".to_string(),
            Self::ErrorOutline => "error_outline".to_string(),
            Self::Escalator => "escalator".to_string(),
            Self::EscalatorWarning => "escalator_warning".to_string(),
            Self::Euro => "euro".to_string(),
            Self::EuroSymbol => "euro_symbol".to_string(),
            Self::EvStation => "ev_station".to_string(),
            Self::Event => "event".to_string(),
            Self::EventAvailable => "event_available".to_string(),
            Self::EventBusy => "event_busy".to_string(),
            Self::EventNote => "event_note".to_string(),
            Self::EventSeat => "event_seat".to_string(),
            Self::ExitToApp => "exit_to_app".to_string(),
            Self::Expand => "expand".to_string(),
            Self::ExpandLess => "expand_less".to_string(),
            Self::ExpandMore => "expand_more".to_string(),
            Self::Explicit => "explicit".to_string(),
            Self::Explore => "explore".to_string(),
            Self::ExploreOff => "explore_off".to_string(),
            Self::Exposure => "exposure".to_string(),
            Self::ExposureMinus1 => "exposure_minus_1".to_string(),
            Self::ExposureMinus2 => "exposure_minus_2".to_string(),
            Self::ExposureNeg1 => "exposure_neg_1".to_string(),
            Self::ExposureNeg2 => "exposure_neg_2".to_string(),
            Self::ExposurePlus1 => "exposure_plus_1".to_string(),
            Self::ExposurePlus2 => "exposure_plus_2".to_string(),
            Self::ExposureZero => "exposure_zero".to_string(),
            Self::Extension => "extension".to_string(),
            Self::ExtensionOff => "extension_off".to_string(),
            Self::Face => "face".to_string(),
            Self::FaceRetouchingOff => "face_retouching_off".to_string(),
            Self::FaceRetouchingNatural => "face_retouching_natural".to_string(),
            Self::Facebook => "facebook".to_string(),
            Self::FactCheck => "fact_check".to_string(),
            Self::FamilyRestroom => "family_restroom".to_string(),
            Self::FastForward => "fast_forward".to_string(),
            Self::FastRewind => "fast_rewind".to_string(),
            Self::Fastfood => "fastfood".to_string(),
            Self::Favorite => "favorite".to_string(),
            Self::FavoriteBorder => "favorite_border".to_string(),
            Self::FavoriteOutline => "favorite_outline".to_string(),
            Self::FeaturedPlayList => "featured_play_list".to_string(),
            Self::FeaturedVideo => "featured_video".to_string(),
            Self::Feed => "feed".to_string(),
            Self::Feedback => "feedback".to_string(),
            Self::Female => "female".to_string(),
            Self::Fence => "fence".to_string(),
            Self::Festival => "festival".to_string(),
            Self::FiberDvr => "fiber_dvr".to_string(),
            Self::FiberManualRecord => "fiber_manual_record".to_string(),
            Self::FiberNew => "fiber_new".to_string(),
            Self::FiberPin => "fiber_pin".to_string(),
            Self::FiberSmartRecord => "fiber_smart_record".to_string(),
            Self::FifteenMp => "fifteen_mp".to_string(),
            Self::FileCopy => "file_copy".to_string(),
            Self::FileDownload => "file_download".to_string(),
            Self::FileDownloadDone => "file_download_done".to_string(),
            Self::FileDownloadOff => "file_download_off".to_string(),
            Self::FilePresent => "file_present".to_string(),
            Self::FileUpload => "file_upload".to_string(),
            Self::Filter => "filter".to_string(),
            Self::Filter1 => "filter_1".to_string(),
            Self::Filter2 => "filter_2".to_string(),
            Self::Filter3 => "filter_3".to_string(),
            Self::Filter4 => "filter_4".to_string(),
            Self::Filter5 => "filter_5".to_string(),
            Self::Filter6 => "filter_6".to_string(),
            Self::Filter7 => "filter_7".to_string(),
            Self::Filter8 => "filter_8".to_string(),
            Self::Filter9 => "filter_9".to_string(),
            Self::Filter9Plus => "filter_9_plus".to_string(),
            Self::FilterAlt => "filter_alt".to_string(),
            Self::FilterBAndW => "filter_b_and_w".to_string(),
            Self::FilterCenterFocus => "filter_center_focus".to_string(),
            Self::FilterDrama => "filter_drama".to_string(),
            Self::FilterFrames => "filter_frames".to_string(),
            Self::FilterHdr => "filter_hdr".to_string(),
            Self::FilterList => "filter_list".to_string(),
            Self::FilterListAlt => "filter_list_alt".to_string(),
            Self::FilterNone => "filter_none".to_string(),
            Self::FilterTiltShift => "filter_tilt_shift".to_string(),
            Self::FilterVintage => "filter_vintage".to_string(),
            Self::FindInPage => "find_in_page".to_string(),
            Self::FindReplace => "find_replace".to_string(),
            Self::Fingerprint => "fingerprint".to_string(),
            Self::FireExtinguisher => "fire_extinguisher".to_string(),
            Self::FireHydrant => "fire_hydrant".to_string(),
            Self::Fireplace => "fireplace".to_string(),
            Self::FirstPage => "first_page".to_string(),
            Self::FitScreen => "fit_screen".to_string(),
            Self::FitnessCenter => "fitness_center".to_string(),
            Self::FiveG => "five_g".to_string(),
            Self::FiveK => "five_k".to_string(),
            Self::FiveKPlus => "five_k_plus".to_string(),
            Self::FiveMp => "five_mp".to_string(),
            Self::Flag => "flag".to_string(),
            Self::Flaky => "flaky".to_string(),
            Self::Flare => "flare".to_string(),
            Self::FlashAuto => "flash_auto".to_string(),
            Self::FlashOff => "flash_off".to_string(),
            Self::FlashOn => "flash_on".to_string(),
            Self::FlashlightOff => "flashlight_off".to_string(),
            Self::FlashlightOn => "flashlight_on".to_string(),
            Self::Flatware => "flatware".to_string(),
            Self::Flight => "flight".to_string(),
            Self::FlightLand => "flight_land".to_string(),
            Self::FlightTakeoff => "flight_takeoff".to_string(),
            Self::Flip => "flip".to_string(),
            Self::FlipCameraAndroid => "flip_camera_android".to_string(),
            Self::FlipCameraIos => "flip_camera_ios".to_string(),
            Self::FlipToBack => "flip_to_back".to_string(),
            Self::FlipToFront => "flip_to_front".to_string(),
            Self::Flourescent => "flourescent".to_string(),
            Self::FlutterDash => "flutter_dash".to_string(),
            Self::FmdBad => "fmd_bad".to_string(),
            Self::FmdGood => "fmd_good".to_string(),
            Self::Folder => "folder".to_string(),
            Self::FolderOpen => "folder_open".to_string(),
            Self::FolderShared => "folder_shared".to_string(),
            Self::FolderSpecial => "folder_special".to_string(),
            Self::FollowTheSigns => "follow_the_signs".to_string(),
            Self::FontDownload => "font_download".to_string(),
            Self::FontDownloadOff => "font_download_off".to_string(),
            Self::FoodBank => "food_bank".to_string(),
            Self::FormatAlignCenter => "format_align_center".to_string(),
            Self::FormatAlignJustify => "format_align_justify".to_string(),
            Self::FormatAlignLeft => "format_align_left".to_string(),
            Self::FormatAlignRight => "format_align_right".to_string(),
            Self::FormatBold => "format_bold".to_string(),
            Self::FormatClear => "format_clear".to_string(),
            Self::FormatColorFill => "format_color_fill".to_string(),
            Self::FormatColorReset => "format_color_reset".to_string(),
            Self::FormatColorText => "format_color_text".to_string(),
            Self::FormatIndentDecrease => "format_indent_decrease".to_string(),
            Self::FormatIndentIncrease => "format_indent_increase".to_string(),
            Self::FormatItalic => "format_italic".to_string(),
            Self::FormatLineSpacing => "format_line_spacing".to_string(),
            Self::FormatListBulleted => "format_list_bulleted".to_string(),
            Self::FormatListNumbered => "format_list_numbered".to_string(),
            Self::FormatListNumberedRtl => "format_list_numbered_rtl".to_string(),
            Self::FormatPaint => "format_paint".to_string(),
            Self::FormatQuote => "format_quote".to_string(),
            Self::FormatShapes => "format_shapes".to_string(),
            Self::FormatSize => "format_size".to_string(),
            Self::FormatStrikethrough => "format_strikethrough".to_string(),
            Self::FormatTextdirectionLToR => "format_textdirection_l_to_r".to_string(),
            Self::FormatTextdirectionRToL => "format_textdirection_r_to_l".to_string(),
            Self::FormatUnderline => "format_underline".to_string(),
            Self::FormatUnderlined => "format_underlined".to_string(),
            Self::Forum => "forum".to_string(),
            Self::Forward => "forward".to_string(),
            Self::Forward10 => "forward_10".to_string(),
            Self::Forward30 => "forward_30".to_string(),
            Self::Forward5 => "forward_5".to_string(),
            Self::ForwardToInbox => "forward_to_inbox".to_string(),
            Self::Foundation => "foundation".to_string(),
            Self::FourGMobiledata => "four_g_mobiledata".to_string(),
            Self::FourGPlusMobiledata => "four_g_plus_mobiledata".to_string(),
            Self::FourK => "four_k".to_string(),
            Self::FourKPlus => "four_k_plus".to_string(),
            Self::FourMp => "four_mp".to_string(),
            Self::FourteenMp => "fourteen_mp".to_string(),
            Self::FreeBreakfast => "free_breakfast".to_string(),
            Self::Fullscreen => "fullscreen".to_string(),
            Self::FullscreenExit => "fullscreen_exit".to_string(),
            Self::Functions => "functions".to_string(),
            Self::GMobiledata => "g_mobiledata".to_string(),
            Self::GTranslate => "g_translate".to_string(),
            Self::Gamepad => "gamepad".to_string(),
            Self::Games => "games".to_string(),
            Self::Garage => "garage".to_string(),
            Self::Gavel => "gavel".to_string(),
            Self::Gesture => "gesture".to_string(),
            Self::GetApp => "get_app".to_string(),
            Self::Gif => "gif".to_string(),
            Self::Gite => "gite".to_string(),
            Self::GolfCourse => "golf_course".to_string(),
            Self::GppBad => "gpp_bad".to_string(),
            Self::GppGood => "gpp_good".to_string(),
            Self::GppMaybe => "gpp_maybe".to_string(),
            Self::GpsFixed => "gps_fixed".to_string(),
            Self::GpsNotFixed => "gps_not_fixed".to_string(),
            Self::GpsOff => "gps_off".to_string(),
            Self::Grade => "grade".to_string(),
            Self::Gradient => "gradient".to_string(),
            Self::Grading => "grading".to_string(),
            Self::Grain => "grain".to_string(),
            Self::GraphicEq => "graphic_eq".to_string(),
            Self::Grass => "grass".to_string(),
            Self::Grid3x3 => "grid_3x3".to_string(),
            Self::Grid4x4 => "grid_4x4".to_string(),
            Self::GridGoldenratio => "grid_goldenratio".to_string(),
            Self::GridOff => "grid_off".to_string(),
            Self::GridOn => "grid_on".to_string(),
            Self::GridView => "grid_view".to_string(),
            Self::Group => "group".to_string(),
            Self::GroupAdd => "group_add".to_string(),
            Self::GroupWork => "group_work".to_string(),
            Self::Groups => "groups".to_string(),
            Self::HMobiledata => "h_mobiledata".to_string(),
            Self::HPlusMobiledata => "h_plus_mobiledata".to_string(),
            Self::Hail => "hail".to_string(),
            Self::Handyman => "handyman".to_string(),
            Self::Hardware => "hardware".to_string(),
            Self::Hd => "hd".to_string(),
            Self::HdrAuto => "hdr_auto".to_string(),
            Self::HdrAutoSelect => "hdr_auto_select".to_string(),
            Self::HdrEnhancedSelect => "hdr_enhanced_select".to_string(),
            Self::HdrOff => "hdr_off".to_string(),
            Self::HdrOffSelect => "hdr_off_select".to_string(),
            Self::HdrOn => "hdr_on".to_string(),
            Self::HdrOnSelect => "hdr_on_select".to_string(),
            Self::HdrPlus => "hdr_plus".to_string(),
            Self::HdrStrong => "hdr_strong".to_string(),
            Self::HdrWeak => "hdr_weak".to_string(),
            Self::Headphones => "headphones".to_string(),
            Self::HeadphonesBattery => "headphones_battery".to_string(),
            Self::Headset => "headset".to_string(),
            Self::HeadsetMic => "headset_mic".to_string(),
            Self::HeadsetOff => "headset_off".to_string(),
            Self::Healing => "healing".to_string(),
            Self::HealthAndSafety => "health_and_safety".to_string(),
            Self::Hearing => "hearing".to_string(),
            Self::HearingDisabled => "hearing_disabled".to_string(),
            Self::Height => "height".to_string(),
            Self::Help => "help".to_string(),
            Self::HelpCenter => "help_center".to_string(),
            Self::HelpOutline => "help_outline".to_string(),
            Self::Hevc => "hevc".to_string(),
            Self::HideImage => "hide_image".to_string(),
            Self::HideSource => "hide_source".to_string(),
            Self::HighQuality => "high_quality".to_string(),
            Self::Highlight => "highlight".to_string(),
            Self::HighlightAlt => "highlight_alt".to_string(),
            Self::HighlightOff => "highlight_off".to_string(),
            Self::HighlightRemove => "highlight_remove".to_string(),
            Self::Hiking => "hiking".to_string(),
            Self::History => "history".to_string(),
            Self::HistoryEdu => "history_edu".to_string(),
            Self::HistoryToggleOff => "history_toggle_off".to_string(),
            Self::HolidayVillage => "holiday_village".to_string(),
            Self::Home => "home".to_string(),
            Self::HomeFilled => "home_filled".to_string(),
            Self::HomeMax => "home_max".to_string(),
            Self::HomeMini => "home_mini".to_string(),
            Self::HomeRepairService => "home_repair_service".to_string(),
            Self::HomeWork => "home_work".to_string(),
            Self::HorizontalDistribute => "horizontal_distribute".to_string(),
            Self::HorizontalRule => "horizontal_rule".to_string(),
            Self::HorizontalSplit => "horizontal_split".to_string(),
            Self::HotTub => "hot_tub".to_string(),
            Self::Hotel => "hotel".to_string(),
            Self::HourglassBottom => "hourglass_bottom".to_string(),
            Self::HourglassDisabled => "hourglass_disabled".to_string(),
            Self::HourglassEmpty => "hourglass_empty".to_string(),
            Self::HourglassFull => "hourglass_full".to_string(),
            Self::HourglassTop => "hourglass_top".to_string(),
            Self::House => "house".to_string(),
            Self::HouseSiding => "house_siding".to_string(),
            Self::Houseboat => "houseboat".to_string(),
            Self::HowToReg => "how_to_reg".to_string(),
            Self::HowToVote => "how_to_vote".to_string(),
            Self::Http => "http".to_string(),
            Self::Https => "https".to_string(),
            Self::Hvac => "hvac".to_string(),
            Self::IceSkating => "ice_skating".to_string(),
            Self::Icecream => "icecream".to_string(),
            Self::Image => "image".to_string(),
            Self::ImageAspectRatio => "image_aspect_ratio".to_string(),
            Self::ImageNotSupported => "image_not_supported".to_string(),
            Self::ImageSearch => "image_search".to_string(),
            Self::ImagesearchRoller => "imagesearch_roller".to_string(),
            Self::ImportContacts => "import_contacts".to_string(),
            Self::ImportExport => "import_export".to_string(),
            Self::ImportantDevices => "important_devices".to_string(),
            Self::Inbox => "inbox".to_string(),
            Self::IndeterminateCheckBox => "indeterminate_check_box".to_string(),
            Self::Info => "info".to_string(),
            Self::InfoOutline => "info_outline".to_string(),
            Self::Input => "input".to_string(),
            Self::InsertChart => "insert_chart".to_string(),
            Self::InsertComment => "insert_comment".to_string(),
            Self::InsertDriveFile => "insert_drive_file".to_string(),
            Self::InsertEmoticon => "insert_emoticon".to_string(),
            Self::InsertInvitation => "insert_invitation".to_string(),
            Self::InsertLink => "insert_link".to_string(),
            Self::InsertPhoto => "insert_photo".to_string(),
            Self::Insights => "insights".to_string(),
            Self::IntegrationInstructions => "integration_instructions".to_string(),
            Self::Inventory => "inventory".to_string(),
            Self::Inventory2 => "inventory_2".to_string(),
            Self::InvertColors => "invert_colors".to_string(),
            Self::InvertColorsOff => "invert_colors_off".to_string(),
            Self::InvertColorsOn => "invert_colors_on".to_string(),
            Self::IosShare => "ios_share".to_string(),
            Self::Iron => "iron".to_string(),
            Self::Iso => "iso".to_string(),
            Self::Kayaking => "kayaking".to_string(),
            Self::Keyboard => "keyboard".to_string(),
            Self::KeyboardAlt => "keyboard_alt".to_string(),
            Self::KeyboardArrowDown => "keyboard_arrow_down".to_string(),
            Self::KeyboardArrowLeft => "keyboard_arrow_left".to_string(),
            Self::KeyboardArrowRight => "keyboard_arrow_right".to_string(),
            Self::KeyboardArrowUp => "keyboard_arrow_up".to_string(),
            Self::KeyboardBackspace => "keyboard_backspace".to_string(),
            Self::KeyboardCapslock => "keyboard_capslock".to_string(),
            Self::KeyboardControl => "keyboard_control".to_string(),
            Self::KeyboardHide => "keyboard_hide".to_string(),
            Self::KeyboardReturn => "keyboard_return".to_string(),
            Self::KeyboardTab => "keyboard_tab".to_string(),
            Self::KeyboardVoice => "keyboard_voice".to_string(),
            Self::KingBed => "king_bed".to_string(),
            Self::Kitchen => "kitchen".to_string(),
            Self::Kitesurfing => "kitesurfing".to_string(),
            Self::Label => "label".to_string(),
            Self::LabelImportant => "label_important".to_string(),
            Self::LabelImportantOutline => "label_important_outline".to_string(),
            Self::LabelOff => "label_off".to_string(),
            Self::LabelOutline => "label_outline".to_string(),
            Self::Landscape => "landscape".to_string(),
            Self::Language => "language".to_string(),
            Self::Laptop => "laptop".to_string(),
            Self::LaptopChromebook => "laptop_chromebook".to_string(),
            Self::LaptopMac => "laptop_mac".to_string(),
            Self::LaptopWindows => "laptop_windows".to_string(),
            Self::LastPage => "last_page".to_string(),
            Self::Launch => "launch".to_string(),
            Self::Layers => "layers".to_string(),
            Self::LayersClear => "layers_clear".to_string(),
            Self::Leaderboard => "leaderboard".to_string(),
            Self::LeakAdd => "leak_add".to_string(),
            Self::LeakRemove => "leak_remove".to_string(),
            Self::LeaveBagsAtHome => "leave_bags_at_home".to_string(),
            Self::LegendToggle => "legend_toggle".to_string(),
            Self::Lens => "lens".to_string(),
            Self::LensBlur => "lens_blur".to_string(),
            Self::LibraryAdd => "library_add".to_string(),
            Self::LibraryAddCheck => "library_add_check".to_string(),
            Self::LibraryBooks => "library_books".to_string(),
            Self::LibraryMusic => "library_music".to_string(),
            Self::Light => "light".to_string(),
            Self::LightMode => "light_mode".to_string(),
            Self::Lightbulb => "lightbulb".to_string(),
            Self::LightbulbOutline => "lightbulb_outline".to_string(),
            Self::LineStyle => "line_style".to_string(),
            Self::LineWeight => "line_weight".to_string(),
            Self::LinearScale => "linear_scale".to_string(),
            Self::Link => "link".to_string(),
            Self::LinkOff => "link_off".to_string(),
            Self::LinkedCamera => "linked_camera".to_string(),
            Self::Liquor => "liquor".to_string(),
            Self::List => "list".to_string(),
            Self::ListAlt => "list_alt".to_string(),
            Self::LiveHelp => "live_help".to_string(),
            Self::LiveTv => "live_tv".to_string(),
            Self::Living => "living".to_string(),
            Self::LocalActivity => "local_activity".to_string(),
            Self::LocalAirport => "local_airport".to_string(),
            Self::LocalAtm => "local_atm".to_string(),
            Self::LocalAttraction => "local_attraction".to_string(),
            Self::LocalBar => "local_bar".to_string(),
            Self::LocalCafe => "local_cafe".to_string(),
            Self::LocalCarWash => "local_car_wash".to_string(),
            Self::LocalConvenienceStore => "local_convenience_store".to_string(),
            Self::LocalDining => "local_dining".to_string(),
            Self::LocalDrink => "local_drink".to_string(),
            Self::LocalFireDepartment => "local_fire_department".to_string(),
            Self::LocalFlorist => "local_florist".to_string(),
            Self::LocalGasStation => "local_gas_station".to_string(),
            Self::LocalGroceryStore => "local_grocery_store".to_string(),
            Self::LocalHospital => "local_hospital".to_string(),
            Self::LocalHotel => "local_hotel".to_string(),
            Self::LocalLaundryService => "local_laundry_service".to_string(),
            Self::LocalLibrary => "local_library".to_string(),
            Self::LocalMall => "local_mall".to_string(),
            Self::LocalMovies => "local_movies".to_string(),
            Self::LocalOffer => "local_offer".to_string(),
            Self::LocalParking => "local_parking".to_string(),
            Self::LocalPharmacy => "local_pharmacy".to_string(),
            Self::LocalPhone => "local_phone".to_string(),
            Self::LocalPizza => "local_pizza".to_string(),
            Self::LocalPlay => "local_play".to_string(),
            Self::LocalPolice => "local_police".to_string(),
            Self::LocalPostOffice => "local_post_office".to_string(),
            Self::LocalPrintShop => "local_print_shop".to_string(),
            Self::LocalPrintshop => "local_printshop".to_string(),
            Self::LocalRestaurant => "local_restaurant".to_string(),
            Self::LocalSee => "local_see".to_string(),
            Self::LocalShipping => "local_shipping".to_string(),
            Self::LocalTaxi => "local_taxi".to_string(),
            Self::LocationCity => "location_city".to_string(),
            Self::LocationDisabled => "location_disabled".to_string(),
            Self::LocationHistory => "location_history".to_string(),
            Self::LocationOff => "location_off".to_string(),
            Self::LocationOn => "location_on".to_string(),
            Self::LocationPin => "location_pin".to_string(),
            Self::LocationSearching => "location_searching".to_string(),
            Self::Lock => "lock".to_string(),
            Self::LockClock => "lock_clock".to_string(),
            Self::LockOpen => "lock_open".to_string(),
            Self::LockOutline => "lock_outline".to_string(),
            Self::Login => "login".to_string(),
            Self::Logout => "logout".to_string(),
            Self::Looks => "looks".to_string(),
            Self::Looks3 => "looks_3".to_string(),
            Self::Looks4 => "looks_4".to_string(),
            Self::Looks5 => "looks_5".to_string(),
            Self::Looks6 => "looks_6".to_string(),
            Self::LooksOne => "looks_one".to_string(),
            Self::LooksTwo => "looks_two".to_string(),
            Self::Loop => "loop".to_string(),
            Self::Loupe => "loupe".to_string(),
            Self::LowPriority => "low_priority".to_string(),
            Self::Loyalty => "loyalty".to_string(),
            Self::LteMobiledata => "lte_mobiledata".to_string(),
            Self::LtePlusMobiledata => "lte_plus_mobiledata".to_string(),
            Self::Luggage => "luggage".to_string(),
            Self::LunchDining => "lunch_dining".to_string(),
            Self::Mail => "mail".to_string(),
            Self::MailOutline => "mail_outline".to_string(),
            Self::Male => "male".to_string(),
            Self::ManageAccounts => "manage_accounts".to_string(),
            Self::ManageSearch => "manage_search".to_string(),
            Self::Map => "map".to_string(),
            Self::MapsHomeWork => "maps_home_work".to_string(),
            Self::MapsUgc => "maps_ugc".to_string(),
            Self::Margin => "margin".to_string(),
            Self::MarkAsUnread => "mark_as_unread".to_string(),
            Self::MarkChatRead => "mark_chat_read".to_string(),
            Self::MarkChatUnread => "mark_chat_unread".to_string(),
            Self::MarkEmailRead => "mark_email_read".to_string(),
            Self::MarkEmailUnread => "mark_email_unread".to_string(),
            Self::Markunread => "markunread".to_string(),
            Self::MarkunreadMailbox => "markunread_mailbox".to_string(),
            Self::Masks => "masks".to_string(),
            Self::Maximize => "maximize".to_string(),
            Self::MediaBluetoothOff => "media_bluetooth_off".to_string(),
            Self::MediaBluetoothOn => "media_bluetooth_on".to_string(),
            Self::Mediation => "mediation".to_string(),
            Self::MedicalServices => "medical_services".to_string(),
            Self::Medication => "medication".to_string(),
            Self::MeetingRoom => "meeting_room".to_string(),
            Self::Memory => "memory".to_string(),
            Self::Menu => "menu".to_string(),
            Self::MenuBook => "menu_book".to_string(),
            Self::MenuOpen => "menu_open".to_string(),
            Self::MergeType => "merge_type".to_string(),
            Self::Message => "message".to_string(),
            Self::Messenger => "messenger".to_string(),
            Self::MessengerOutline => "messenger_outline".to_string(),
            Self::Mic => "mic".to_string(),
            Self::MicExternalOff => "mic_external_off".to_string(),
            Self::MicExternalOn => "mic_external_on".to_string(),
            Self::MicNone => "mic_none".to_string(),
            Self::MicOff => "mic_off".to_string(),
            Self::Microwave => "microwave".to_string(),
            Self::MilitaryTech => "military_tech".to_string(),
            Self::Minimize => "minimize".to_string(),
            Self::MiscellaneousServices => "miscellaneous_services".to_string(),
            Self::MissedVideoCall => "missed_video_call".to_string(),
            Self::Mms => "mms".to_string(),
            Self::MobileFriendly => "mobile_friendly".to_string(),
            Self::MobileOff => "mobile_off".to_string(),
            Self::MobileScreenShare => "mobile_screen_share".to_string(),
            Self::MobiledataOff => "mobiledata_off".to_string(),
            Self::Mode => "mode".to_string(),
            Self::ModeComment => "mode_comment".to_string(),
            Self::ModeEdit => "mode_edit".to_string(),
            Self::ModeEditOutline => "mode_edit_outline".to_string(),
            Self::ModeNight => "mode_night".to_string(),
            Self::ModeStandby => "mode_standby".to_string(),
            Self::ModelTraining => "model_training".to_string(),
            Self::MonetizationOn => "monetization_on".to_string(),
            Self::Money => "money".to_string(),
            Self::MoneyOff => "money_off".to_string(),
            Self::MoneyOffCsred => "money_off_csred".to_string(),
            Self::Monitor => "monitor".to_string(),
            Self::MonitorWeight => "monitor_weight".to_string(),
            Self::MonochromePhotos => "monochrome_photos".to_string(),
            Self::Mood => "mood".to_string(),
            Self::MoodBad => "mood_bad".to_string(),
            Self::Moped => "moped".to_string(),
            Self::More => "more".to_string(),
            Self::MoreHoriz => "more_horiz".to_string(),
            Self::MoreTime => "more_time".to_string(),
            Self::MoreVert => "more_vert".to_string(),
            Self::MotionPhotosAuto => "motion_photos_auto".to_string(),
            Self::MotionPhotosOff => "motion_photos_off".to_string(),
            Self::MotionPhotosOn => "motion_photos_on".to_string(),
            Self::MotionPhotosPause => "motion_photos_pause".to_string(),
            Self::MotionPhotosPaused => "motion_photos_paused".to_string(),
            Self::Motorcycle => "motorcycle".to_string(),
            Self::Mouse => "mouse".to_string(),
            Self::MoveToInbox => "move_to_inbox".to_string(),
            Self::Movie => "movie".to_string(),
            Self::MovieCreation => "movie_creation".to_string(),
            Self::MovieFilter => "movie_filter".to_string(),
            Self::Moving => "moving".to_string(),
            Self::Mp => "mp".to_string(),
            Self::MultilineChart => "multiline_chart".to_string(),
            Self::MultipleStop => "multiple_stop".to_string(),
            Self::MultitrackAudio => "multitrack_audio".to_string(),
            Self::Museum => "museum".to_string(),
            Self::MusicNote => "music_note".to_string(),
            Self::MusicOff => "music_off".to_string(),
            Self::MusicVideo => "music_video".to_string(),
            Self::MyLibraryAdd => "my_library_add".to_string(),
            Self::MyLibraryBooks => "my_library_books".to_string(),
            Self::MyLibraryMusic => "my_library_music".to_string(),
            Self::MyLocation => "my_location".to_string(),
            Self::Nat => "nat".to_string(),
            Self::Nature => "nature".to_string(),
            Self::NaturePeople => "nature_people".to_string(),
            Self::NavigateBefore => "navigate_before".to_string(),
            Self::NavigateNext => "navigate_next".to_string(),
            Self::Navigation => "navigation".to_string(),
            Self::NearMe => "near_me".to_string(),
            Self::NearMeDisabled => "near_me_disabled".to_string(),
            Self::NearbyError => "nearby_error".to_string(),
            Self::NearbyOff => "nearby_off".to_string(),
            Self::NetworkCell => "network_cell".to_string(),
            Self::NetworkCheck => "network_check".to_string(),
            Self::NetworkLocked => "network_locked".to_string(),
            Self::NetworkWifi => "network_wifi".to_string(),
            Self::NewLabel => "new_label".to_string(),
            Self::NewReleases => "new_releases".to_string(),
            Self::NextPlan => "next_plan".to_string(),
            Self::NextWeek => "next_week".to_string(),
            Self::Nfc => "nfc".to_string(),
            Self::NightShelter => "night_shelter".to_string(),
            Self::Nightlife => "nightlife".to_string(),
            Self::Nightlight => "nightlight".to_string(),
            Self::NightlightRound => "nightlight_round".to_string(),
            Self::NightsStay => "nights_stay".to_string(),
            Self::NineK => "nine_k".to_string(),
            Self::NineKPlus => "nine_k_plus".to_string(),
            Self::NineMp => "nine_mp".to_string(),
            Self::NineteenMp => "nineteen_mp".to_string(),
            Self::NoAccounts => "no_accounts".to_string(),
            Self::NoBackpack => "no_backpack".to_string(),
            Self::NoCell => "no_cell".to_string(),
            Self::NoDrinks => "no_drinks".to_string(),
            Self::NoEncryption => "no_encryption".to_string(),
            Self::NoEncryptionGmailerrorred => "no_encryption_gmailerrorred".to_string(),
            Self::NoFlash => "no_flash".to_string(),
            Self::NoFood => "no_food".to_string(),
            Self::NoLuggage => "no_luggage".to_string(),
            Self::NoMeals => "no_meals".to_string(),
            Self::NoMealsOuline => "no_meals_ouline".to_string(),
            Self::NoMeetingRoom => "no_meeting_room".to_string(),
            Self::NoPhotography => "no_photography".to_string(),
            Self::NoSim => "no_sim".to_string(),
            Self::NoStroller => "no_stroller".to_string(),
            Self::NoTransfer => "no_transfer".to_string(),
            Self::NordicWalking => "nordic_walking".to_string(),
            Self::North => "north".to_string(),
            Self::NorthEast => "north_east".to_string(),
            Self::NorthWest => "north_west".to_string(),
            Self::NotAccessible => "not_accessible".to_string(),
            Self::NotInterested => "not_interested".to_string(),
            Self::NotListedLocation => "not_listed_location".to_string(),
            Self::NotStarted => "not_started".to_string(),
            Self::Note => "note".to_string(),
            Self::NoteAdd => "note_add".to_string(),
            Self::NoteAlt => "note_alt".to_string(),
            Self::Notes => "notes".to_string(),
            Self::NotificationAdd => "notification_add".to_string(),
            Self::NotificationImportant => "notification_important".to_string(),
            Self::Notifications => "notifications".to_string(),
            Self::NotificationsActive => "notifications_active".to_string(),
            Self::NotificationsNone => "notifications_none".to_string(),
            Self::NotificationsOff => "notifications_off".to_string(),
            Self::NotificationsOn => "notifications_on".to_string(),
            Self::NotificationsPaused => "notifications_paused".to_string(),
            Self::NowWallpaper => "now_wallpaper".to_string(),
            Self::NowWidgets => "now_widgets".to_string(),
            Self::OfflineBolt => "offline_bolt".to_string(),
            Self::OfflinePin => "offline_pin".to_string(),
            Self::OfflineShare => "offline_share".to_string(),
            Self::OndemandVideo => "ondemand_video".to_string(),
            Self::OneK => "one_k".to_string(),
            Self::OneKPlus => "one_k_plus".to_string(),
            Self::OneXMobiledata => "one_x_mobiledata".to_string(),
            Self::OnlinePrediction => "online_prediction".to_string(),
            Self::Opacity => "opacity".to_string(),
            Self::OpenInBrowser => "open_in_browser".to_string(),
            Self::OpenInFull => "open_in_full".to_string(),
            Self::OpenInNew => "open_in_new".to_string(),
            Self::OpenInNewOff => "open_in_new_off".to_string(),
            Self::OpenWith => "open_with".to_string(),
            Self::OtherHouses => "other_houses".to_string(),
            Self::Outbond => "outbond".to_string(),
            Self::Outbound => "outbound".to_string(),
            Self::Outbox => "outbox".to_string(),
            Self::OutdoorGrill => "outdoor_grill".to_string(),
            Self::OutgoingMail => "outgoing_mail".to_string(),
            Self::Outlet => "outlet".to_string(),
            Self::OutlinedFlag => "outlined_flag".to_string(),
            Self::Padding => "padding".to_string(),
            Self::Pages => "pages".to_string(),
            Self::Pageview => "pageview".to_string(),
            Self::Paid => "paid".to_string(),
            Self::Palette => "palette".to_string(),
            Self::PanTool => "pan_tool".to_string(),
            Self::Panorama => "panorama".to_string(),
            Self::PanoramaFishEye => "panorama_fish_eye".to_string(),
            Self::PanoramaFisheye => "panorama_fisheye".to_string(),
            Self::PanoramaHorizontal => "panorama_horizontal".to_string(),
            Self::PanoramaHorizontalSelect => "panorama_horizontal_select".to_string(),
            Self::PanoramaPhotosphere => "panorama_photosphere".to_string(),
            Self::PanoramaPhotosphereSelect => "panorama_photosphere_select".to_string(),
            Self::PanoramaVertical => "panorama_vertical".to_string(),
            Self::PanoramaVerticalSelect => "panorama_vertical_select".to_string(),
            Self::PanoramaWideAngle => "panorama_wide_angle".to_string(),
            Self::PanoramaWideAngleSelect => "panorama_wide_angle_select".to_string(),
            Self::Paragliding => "paragliding".to_string(),
            Self::Park => "park".to_string(),
            Self::PartyMode => "party_mode".to_string(),
            Self::Password => "password".to_string(),
            Self::Paste => "paste".to_string(),
            Self::Pattern => "pattern".to_string(),
            Self::Pause => "pause".to_string(),
            Self::PauseCircle => "pause_circle".to_string(),
            Self::PauseCircleFilled => "pause_circle_filled".to_string(),
            Self::PauseCircleOutline => "pause_circle_outline".to_string(),
            Self::PausePresentation => "pause_presentation".to_string(),
            Self::Payment => "payment".to_string(),
            Self::Payments => "payments".to_string(),
            Self::PedalBike => "pedal_bike".to_string(),
            Self::Pending => "pending".to_string(),
            Self::PendingActions => "pending_actions".to_string(),
            Self::People => "people".to_string(),
            Self::PeopleAlt => "people_alt".to_string(),
            Self::PeopleOutline => "people_outline".to_string(),
            Self::PermCameraMic => "perm_camera_mic".to_string(),
            Self::PermContactCal => "perm_contact_cal".to_string(),
            Self::PermContactCalendar => "perm_contact_calendar".to_string(),
            Self::PermDataSetting => "perm_data_setting".to_string(),
            Self::PermDeviceInfo => "perm_device_info".to_string(),
            Self::PermDeviceInformation => "perm_device_information".to_string(),
            Self::PermIdentity => "perm_identity".to_string(),
            Self::PermMedia => "perm_media".to_string(),
            Self::PermPhoneMsg => "perm_phone_msg".to_string(),
            Self::PermScanWifi => "perm_scan_wifi".to_string(),
            Self::Person => "person".to_string(),
            Self::PersonAdd => "person_add".to_string(),
            Self::PersonAddAlt => "person_add_alt".to_string(),
            Self::PersonAddAlt1 => "person_add_alt_1".to_string(),
            Self::PersonAddDisabled => "person_add_disabled".to_string(),
            Self::PersonOff => "person_off".to_string(),
            Self::PersonOutline => "person_outline".to_string(),
            Self::PersonPin => "person_pin".to_string(),
            Self::PersonPinCircle => "person_pin_circle".to_string(),
            Self::PersonRemove => "person_remove".to_string(),
            Self::PersonRemoveAlt1 => "person_remove_alt_1".to_string(),
            Self::PersonSearch => "person_search".to_string(),
            Self::PersonalInjury => "personal_injury".to_string(),
            Self::PersonalVideo => "personal_video".to_string(),
            Self::PestControl => "pest_control".to_string(),
            Self::PestControlRodent => "pest_control_rodent".to_string(),
            Self::Pets => "pets".to_string(),
            Self::Phone => "phone".to_string(),
            Self::PhoneAndroid => "phone_android".to_string(),
            Self::PhoneBluetoothSpeaker => "phone_bluetooth_speaker".to_string(),
            Self::PhoneCallback => "phone_callback".to_string(),
            Self::PhoneDisabled => "phone_disabled".to_string(),
            Self::PhoneEnabled => "phone_enabled".to_string(),
            Self::PhoneForwarded => "phone_forwarded".to_string(),
            Self::PhoneInTalk => "phone_in_talk".to_string(),
            Self::PhoneIphone => "phone_iphone".to_string(),
            Self::PhoneLocked => "phone_locked".to_string(),
            Self::PhoneMissed => "phone_missed".to_string(),
            Self::PhonePaused => "phone_paused".to_string(),
            Self::Phonelink => "phonelink".to_string(),
            Self::PhonelinkErase => "phonelink_erase".to_string(),
            Self::PhonelinkLock => "phonelink_lock".to_string(),
            Self::PhonelinkOff => "phonelink_off".to_string(),
            Self::PhonelinkRing => "phonelink_ring".to_string(),
            Self::PhonelinkSetup => "phonelink_setup".to_string(),
            Self::Photo => "photo".to_string(),
            Self::PhotoAlbum => "photo_album".to_string(),
            Self::PhotoCamera => "photo_camera".to_string(),
            Self::PhotoCameraBack => "photo_camera_back".to_string(),
            Self::PhotoCameraFront => "photo_camera_front".to_string(),
            Self::PhotoFilter => "photo_filter".to_string(),
            Self::PhotoLibrary => "photo_library".to_string(),
            Self::PhotoSizeSelectActual => "photo_size_select_actual".to_string(),
            Self::PhotoSizeSelectLarge => "photo_size_select_large".to_string(),
            Self::PhotoSizeSelectSmall => "photo_size_select_small".to_string(),
            Self::Piano => "piano".to_string(),
            Self::PianoOff => "piano_off".to_string(),
            Self::PictureAsPdf => "picture_as_pdf".to_string(),
            Self::PictureInPicture => "picture_in_picture".to_string(),
            Self::PictureInPictureAlt => "picture_in_picture_alt".to_string(),
            Self::PieChart => "pie_chart".to_string(),
            Self::PieChartOutline => "pie_chart_outline".to_string(),
            Self::Pin => "pin".to_string(),
            Self::PinDrop => "pin_drop".to_string(),
            Self::PivotTableChart => "pivot_table_chart".to_string(),
            Self::Place => "place".to_string(),
            Self::Plagiarism => "plagiarism".to_string(),
            Self::PlayArrow => "play_arrow".to_string(),
            Self::PlayCircle => "play_circle".to_string(),
            Self::PlayCircleFill => "play_circle_fill".to_string(),
            Self::PlayCircleFilled => "play_circle_filled".to_string(),
            Self::PlayCircleOutline => "play_circle_outline".to_string(),
            Self::PlayDisabled => "play_disabled".to_string(),
            Self::PlayForWork => "play_for_work".to_string(),
            Self::PlayLesson => "play_lesson".to_string(),
            Self::PlaylistAdd => "playlist_add".to_string(),
            Self::PlaylistAddCheck => "playlist_add_check".to_string(),
            Self::PlaylistPlay => "playlist_play".to_string(),
            Self::Plumbing => "plumbing".to_string(),
            Self::PlusOne => "plus_one".to_string(),
            Self::Podcasts => "podcasts".to_string(),
            Self::PointOfSale => "point_of_sale".to_string(),
            Self::Policy => "policy".to_string(),
            Self::Poll => "poll".to_string(),
            Self::Polymer => "polymer".to_string(),
            Self::Pool => "pool".to_string(),
            Self::PortableWifiOff => "portable_wifi_off".to_string(),
            Self::Portrait => "portrait".to_string(),
            Self::PostAdd => "post_add".to_string(),
            Self::Power => "power".to_string(),
            Self::PowerInput => "power_input".to_string(),
            Self::PowerOff => "power_off".to_string(),
            Self::PowerSettingsNew => "power_settings_new".to_string(),
            Self::PrecisionManufacturing => "precision_manufacturing".to_string(),
            Self::PregnantWoman => "pregnant_woman".to_string(),
            Self::PresentToAll => "present_to_all".to_string(),
            Self::Preview => "preview".to_string(),
            Self::PriceChange => "price_change".to_string(),
            Self::PriceCheck => "price_check".to_string(),
            Self::Print => "print".to_string(),
            Self::PrintDisabled => "print_disabled".to_string(),
            Self::PriorityHigh => "priority_high".to_string(),
            Self::PrivacyTip => "privacy_tip".to_string(),
            Self::ProductionQuantityLimits => "production_quantity_limits".to_string(),
            Self::Psychology => "psychology".to_string(),
            Self::Public => "public".to_string(),
            Self::PublicOff => "public_off".to_string(),
            Self::Publish => "publish".to_string(),
            Self::PublishedWithChanges => "published_with_changes".to_string(),
            Self::PushPin => "push_pin".to_string(),
            Self::QrCode => "qr_code".to_string(),
            Self::QrCode2 => "qr_code_2".to_string(),
            Self::QrCodeScanner => "qr_code_scanner".to_string(),
            Self::QueryBuilder => "query_builder".to_string(),
            Self::QueryStats => "query_stats".to_string(),
            Self::QuestionAnswer => "question_answer".to_string(),
            Self::Queue => "queue".to_string(),
            Self::QueueMusic => "queue_music".to_string(),
            Self::QueuePlayNext => "queue_play_next".to_string(),
            Self::QuickContactsDialer => "quick_contacts_dialer".to_string(),
            Self::QuickContactsMail => "quick_contacts_mail".to_string(),
            Self::Quickreply => "quickreply".to_string(),
            Self::Quiz => "quiz".to_string(),
            Self::RMobiledata => "r_mobiledata".to_string(),
            Self::Radar => "radar".to_string(),
            Self::Radio => "radio".to_string(),
            Self::RadioButtonChecked => "radio_button_checked".to_string(),
            Self::RadioButtonOff => "radio_button_off".to_string(),
            Self::RadioButtonOn => "radio_button_on".to_string(),
            Self::RadioButtonUnchecked => "radio_button_unchecked".to_string(),
            Self::RailwayAlert => "railway_alert".to_string(),
            Self::RamenDining => "ramen_dining".to_string(),
            Self::RateReview => "rate_review".to_string(),
            Self::RawOff => "raw_off".to_string(),
            Self::RawOn => "raw_on".to_string(),
            Self::ReadMore => "read_more".to_string(),
            Self::RealEstateAgent => "real_estate_agent".to_string(),
            Self::Receipt => "receipt".to_string(),
            Self::ReceiptLong => "receipt_long".to_string(),
            Self::RecentActors => "recent_actors".to_string(),
            Self::Recommend => "recommend".to_string(),
            Self::RecordVoiceOver => "record_voice_over".to_string(),
            Self::Redeem => "redeem".to_string(),
            Self::Redo => "redo".to_string(),
            Self::ReduceCapacity => "reduce_capacity".to_string(),
            Self::Refresh => "refresh".to_string(),
            Self::RememberMe => "remember_me".to_string(),
            Self::Remove => "remove".to_string(),
            Self::RemoveCircle => "remove_circle".to_string(),
            Self::RemoveCircleOutline => "remove_circle_outline".to_string(),
            Self::RemoveDone => "remove_done".to_string(),
            Self::RemoveFromQueue => "remove_from_queue".to_string(),
            Self::RemoveModerator => "remove_moderator".to_string(),
            Self::RemoveRedEye => "remove_red_eye".to_string(),
            Self::RemoveShoppingCart => "remove_shopping_cart".to_string(),
            Self::Reorder => "reorder".to_string(),
            Self::Repeat => "repeat".to_string(),
            Self::RepeatOn => "repeat_on".to_string(),
            Self::RepeatOne => "repeat_one".to_string(),
            Self::RepeatOneOn => "repeat_one_on".to_string(),
            Self::Replay => "replay".to_string(),
            Self::Replay10 => "replay_10".to_string(),
            Self::Replay30 => "replay_30".to_string(),
            Self::Replay5 => "replay_5".to_string(),
            Self::ReplayCircleFilled => "replay_circle_filled".to_string(),
            Self::Reply => "reply".to_string(),
            Self::ReplyAll => "reply_all".to_string(),
            Self::Report => "report".to_string(),
            Self::ReportGmailerrorred => "report_gmailerrorred".to_string(),
            Self::ReportOff => "report_off".to_string(),
            Self::ReportProblem => "report_problem".to_string(),
            Self::RequestPage => "request_page".to_string(),
            Self::RequestQuote => "request_quote".to_string(),
            Self::ResetTv => "reset_tv".to_string(),
            Self::RestartAlt => "restart_alt".to_string(),
            Self::Restaurant => "restaurant".to_string(),
            Self::RestaurantMenu => "restaurant_menu".to_string(),
            Self::Restore => "restore".to_string(),
            Self::RestoreFromTrash => "restore_from_trash".to_string(),
            Self::RestorePage => "restore_page".to_string(),
            Self::Reviews => "reviews".to_string(),
            Self::RiceBowl => "rice_bowl".to_string(),
            Self::RingVolume => "ring_volume".to_string(),
            Self::Roofing => "roofing".to_string(),
            Self::Room => "room".to_string(),
            Self::RoomPreferences => "room_preferences".to_string(),
            Self::RoomService => "room_service".to_string(),
            Self::Rotate90DegreesCcw => "rotate_90_degrees_ccw".to_string(),
            Self::RotateLeft => "rotate_left".to_string(),
            Self::RotateRight => "rotate_right".to_string(),
            Self::RoundedCorner => "rounded_corner".to_string(),
            Self::Router => "router".to_string(),
            Self::Rowing => "rowing".to_string(),
            Self::RssFeed => "rss_feed".to_string(),
            Self::Rsvp => "rsvp".to_string(),
            Self::Rtt => "rtt".to_string(),
            Self::Rule => "rule".to_string(),
            Self::RuleFolder => "rule_folder".to_string(),
            Self::RunCircle => "run_circle".to_string(),
            Self::RunningWithErrors => "running_with_errors".to_string(),
            Self::RvHookup => "rv_hookup".to_string(),
            Self::SafetyDivider => "safety_divider".to_string(),
            Self::Sailing => "sailing".to_string(),
            Self::Sanitizer => "sanitizer".to_string(),
            Self::Satellite => "satellite".to_string(),
            Self::Save => "save".to_string(),
            Self::SaveAlt => "save_alt".to_string(),
            Self::SavedSearch => "saved_search".to_string(),
            Self::Savings => "savings".to_string(),
            Self::Scanner => "scanner".to_string(),
            Self::ScatterPlot => "scatter_plot".to_string(),
            Self::Schedule => "schedule".to_string(),
            Self::ScheduleSend => "schedule_send".to_string(),
            Self::Schema => "schema".to_string(),
            Self::School => "school".to_string(),
            Self::Science => "science".to_string(),
            Self::Score => "score".to_string(),
            Self::ScreenLockLandscape => "screen_lock_landscape".to_string(),
            Self::ScreenLockPortrait => "screen_lock_portrait".to_string(),
            Self::ScreenLockRotation => "screen_lock_rotation".to_string(),
            Self::ScreenRotation => "screen_rotation".to_string(),
            Self::ScreenSearchDesktop => "screen_search_desktop".to_string(),
            Self::ScreenShare => "screen_share".to_string(),
            Self::Screenshot => "screenshot".to_string(),
            Self::Sd => "sd".to_string(),
            Self::SdCard => "sd_card".to_string(),
            Self::SdCardAlert => "sd_card_alert".to_string(),
            Self::SdStorage => "sd_storage".to_string(),
            Self::Search => "search".to_string(),
            Self::SearchOff => "search_off".to_string(),
            Self::Security => "security".to_string(),
            Self::SecurityUpdate => "security_update".to_string(),
            Self::SecurityUpdateGood => "security_update_good".to_string(),
            Self::SecurityUpdateWarning => "security_update_warning".to_string(),
            Self::Segment => "segment".to_string(),
            Self::SelectAll => "select_all".to_string(),
            Self::SelfImprovement => "self_improvement".to_string(),
            Self::Sell => "sell".to_string(),
            Self::Send => "send".to_string(),
            Self::SendAndArchive => "send_and_archive".to_string(),
            Self::SendToMobile => "send_to_mobile".to_string(),
            Self::SensorDoor => "sensor_door".to_string(),
            Self::SensorWindow => "sensor_window".to_string(),
            Self::Sensors => "sensors".to_string(),
            Self::SensorsOff => "sensors_off".to_string(),
            Self::SentimentDissatisfied => "sentiment_dissatisfied".to_string(),
            Self::SentimentNeutral => "sentiment_neutral".to_string(),
            Self::SentimentSatisfied => "sentiment_satisfied".to_string(),
            Self::SentimentSatisfiedAlt => "sentiment_satisfied_alt".to_string(),
            Self::SentimentVeryDissatisfied => "sentiment_very_dissatisfied".to_string(),
            Self::SentimentVerySatisfied => "sentiment_very_satisfied".to_string(),
            Self::SetMeal => "set_meal".to_string(),
            Self::Settings => "settings".to_string(),
            Self::SettingsAccessibility => "settings_accessibility".to_string(),
            Self::SettingsApplications => "settings_applications".to_string(),
            Self::SettingsBackupRestore => "settings_backup_restore".to_string(),
            Self::SettingsBluetooth => "settings_bluetooth".to_string(),
            Self::SettingsBrightness => "settings_brightness".to_string(),
            Self::SettingsCell => "settings_cell".to_string(),
            Self::SettingsDisplay => "settings_display".to_string(),
            Self::SettingsEthernet => "settings_ethernet".to_string(),
            Self::SettingsInputAntenna => "settings_input_antenna".to_string(),
            Self::SettingsInputComponent => "settings_input_component".to_string(),
            Self::SettingsInputComposite => "settings_input_composite".to_string(),
            Self::SettingsInputHdmi => "settings_input_hdmi".to_string(),
            Self::SettingsInputSvideo => "settings_input_svideo".to_string(),
            Self::SettingsOverscan => "settings_overscan".to_string(),
            Self::SettingsPhone => "settings_phone".to_string(),
            Self::SettingsPower => "settings_power".to_string(),
            Self::SettingsRemote => "settings_remote".to_string(),
            Self::SettingsSuggest => "settings_suggest".to_string(),
            Self::SettingsSystemDaydream => "settings_system_daydream".to_string(),
            Self::SettingsVoice => "settings_voice".to_string(),
            Self::SevenK => "seven_k".to_string(),
            Self::SevenKPlus => "seven_k_plus".to_string(),
            Self::SevenMp => "seven_mp".to_string(),
            Self::SeventeenMp => "seventeen_mp".to_string(),
            Self::Share => "share".to_string(),
            Self::ShareArrivalTime => "share_arrival_time".to_string(),
            Self::ShareLocation => "share_location".to_string(),
            Self::Shield => "shield".to_string(),
            Self::Shop => "shop".to_string(),
            Self::Shop2 => "shop_2".to_string(),
            Self::ShopTwo => "shop_two".to_string(),
            Self::ShoppingBag => "shopping_bag".to_string(),
            Self::ShoppingBasket => "shopping_basket".to_string(),
            Self::ShoppingCart => "shopping_cart".to_string(),
            Self::ShortText => "short_text".to_string(),
            Self::Shortcut => "shortcut".to_string(),
            Self::ShowChart => "show_chart".to_string(),
            Self::Shower => "shower".to_string(),
            Self::Shuffle => "shuffle".to_string(),
            Self::ShuffleOn => "shuffle_on".to_string(),
            Self::ShutterSpeed => "shutter_speed".to_string(),
            Self::Sick => "sick".to_string(),
            Self::SignalCellular0Bar => "signal_cellular_0_bar".to_string(),
            Self::SignalCellular4Bar => "signal_cellular_4_bar".to_string(),
            Self::SignalCellularAlt => "signal_cellular_alt".to_string(),
            Self::SignalCellularConnectedNoInternet0Bar => {
                "signal_cellular_connected_no_internet_0_bar".to_string()
            }
            Self::SignalCellularConnectedNoInternet4Bar => {
                "signal_cellular_connected_no_internet_4_bar".to_string()
            }
            Self::SignalCellularNoSim => "signal_cellular_no_sim".to_string(),
            Self::SignalCellularNodata => "signal_cellular_nodata".to_string(),
            Self::SignalCellularNull => "signal_cellular_null".to_string(),
            Self::SignalCellularOff => "signal_cellular_off".to_string(),
            Self::SignalWifi0Bar => "signal_wifi_0_bar".to_string(),
            Self::SignalWifi4Bar => "signal_wifi_4_bar".to_string(),
            Self::SignalWifi4BarLock => "signal_wifi_4_bar_lock".to_string(),
            Self::SignalWifiBad => "signal_wifi_bad".to_string(),
            Self::SignalWifiConnectedNoInternet4 => {
                "signal_wifi_connected_no_internet_4".to_string()
            }
            Self::SignalWifiOff => "signal_wifi_off".to_string(),
            Self::SignalWifiStatusbar4Bar => "signal_wifi_statusbar_4_bar".to_string(),
            Self::SignalWifiStatusbarConnectedNoInternet4 => {
                "signal_wifi_statusbar_connected_no_internet_4".to_string()
            }
            Self::SignalWifiStatusbarNull => "signal_wifi_statusbar_null".to_string(),
            Self::SimCard => "sim_card".to_string(),
            Self::SimCardAlert => "sim_card_alert".to_string(),
            Self::SimCardDownload => "sim_card_download".to_string(),
            Self::SingleBed => "single_bed".to_string(),
            Self::Sip => "sip".to_string(),
            Self::SixFtApart => "six_ft_apart".to_string(),
            Self::SixK => "six_k".to_string(),
            Self::SixKPlus => "six_k_plus".to_string(),
            Self::SixMp => "six_mp".to_string(),
            Self::SixteenMp => "sixteen_mp".to_string(),
            Self::SixtyFps => "sixty_fps".to_string(),
            Self::SixtyFpsSelect => "sixty_fps_select".to_string(),
            Self::Skateboarding => "skateboarding".to_string(),
            Self::SkipNext => "skip_next".to_string(),
            Self::SkipPrevious => "skip_previous".to_string(),
            Self::Sledding => "sledding".to_string(),
            Self::Slideshow => "slideshow".to_string(),
            Self::SlowMotionVideo => "slow_motion_video".to_string(),
            Self::SmartButton => "smart_button".to_string(),
            Self::SmartDisplay => "smart_display".to_string(),
            Self::SmartScreen => "smart_screen".to_string(),
            Self::SmartToy => "smart_toy".to_string(),
            Self::Smartphone => "smartphone".to_string(),
            Self::SmokeFree => "smoke_free".to_string(),
            Self::SmokingRooms => "smoking_rooms".to_string(),
            Self::Sms => "sms".to_string(),
            Self::SmsFailed => "sms_failed".to_string(),
            Self::SnippetFolder => "snippet_folder".to_string(),
            Self::Snooze => "snooze".to_string(),
            Self::Snowboarding => "snowboarding".to_string(),
            Self::Snowmobile => "snowmobile".to_string(),
            Self::Snowshoeing => "snowshoeing".to_string(),
            Self::Soap => "soap".to_string(),
            Self::SocialDistance => "social_distance".to_string(),
            Self::Sort => "sort".to_string(),
            Self::SortByAlpha => "sort_by_alpha".to_string(),
            Self::Source => "source".to_string(),
            Self::South => "south".to_string(),
            Self::SouthEast => "south_east".to_string(),
            Self::SouthWest => "south_west".to_string(),
            Self::Spa => "spa".to_string(),
            Self::SpaceBar => "space_bar".to_string(),
            Self::SpaceDashboard => "space_dashboard".to_string(),
            Self::Speaker => "speaker".to_string(),
            Self::SpeakerGroup => "speaker_group".to_string(),
            Self::SpeakerNotes => "speaker_notes".to_string(),
            Self::SpeakerNotesOff => "speaker_notes_off".to_string(),
            Self::SpeakerPhone => "speaker_phone".to_string(),
            Self::Speed => "speed".to_string(),
            Self::Spellcheck => "spellcheck".to_string(),
            Self::Splitscreen => "splitscreen".to_string(),
            Self::Sports => "sports".to_string(),
            Self::SportsBar => "sports_bar".to_string(),
            Self::SportsBaseball => "sports_baseball".to_string(),
            Self::SportsBasketball => "sports_basketball".to_string(),
            Self::SportsCricket => "sports_cricket".to_string(),
            Self::SportsEsports => "sports_esports".to_string(),
            Self::SportsFootball => "sports_football".to_string(),
            Self::SportsGolf => "sports_golf".to_string(),
            Self::SportsHandball => "sports_handball".to_string(),
            Self::SportsHockey => "sports_hockey".to_string(),
            Self::SportsKabaddi => "sports_kabaddi".to_string(),
            Self::SportsMma => "sports_mma".to_string(),
            Self::SportsMotorsports => "sports_motorsports".to_string(),
            Self::SportsRugby => "sports_rugby".to_string(),
            Self::SportsScore => "sports_score".to_string(),
            Self::SportsSoccer => "sports_soccer".to_string(),
            Self::SportsTennis => "sports_tennis".to_string(),
            Self::SportsVolleyball => "sports_volleyball".to_string(),
            Self::SquareFoot => "square_foot".to_string(),
            Self::StackedBarChart => "stacked_bar_chart".to_string(),
            Self::StackedLineChart => "stacked_line_chart".to_string(),
            Self::Stairs => "stairs".to_string(),
            Self::Star => "star".to_string(),
            Self::StarBorder => "star_border".to_string(),
            Self::StarBorderPurple500 => "star_border_purple500".to_string(),
            Self::StarHalf => "star_half".to_string(),
            Self::StarOutline => "star_outline".to_string(),
            Self::StarPurple500 => "star_purple500".to_string(),
            Self::StarRate => "star_rate".to_string(),
            Self::Stars => "stars".to_string(),
            Self::StayCurrentLandscape => "stay_current_landscape".to_string(),
            Self::StayCurrentPortrait => "stay_current_portrait".to_string(),
            Self::StayPrimaryLandscape => "stay_primary_landscape".to_string(),
            Self::StayPrimaryPortrait => "stay_primary_portrait".to_string(),
            Self::StickyNote2 => "sticky_note_2".to_string(),
            Self::Stop => "stop".to_string(),
            Self::StopCircle => "stop_circle".to_string(),
            Self::StopScreenShare => "stop_screen_share".to_string(),
            Self::Storage => "storage".to_string(),
            Self::Store => "store".to_string(),
            Self::StoreMallDirectory => "store_mall_directory".to_string(),
            Self::Storefront => "storefront".to_string(),
            Self::Storm => "storm".to_string(),
            Self::Straighten => "straighten".to_string(),
            Self::Stream => "stream".to_string(),
            Self::Streetview => "streetview".to_string(),
            Self::StrikethroughS => "strikethrough_s".to_string(),
            Self::Stroller => "stroller".to_string(),
            Self::Style => "style".to_string(),
            Self::SubdirectoryArrowLeft => "subdirectory_arrow_left".to_string(),
            Self::SubdirectoryArrowRight => "subdirectory_arrow_right".to_string(),
            Self::Subject => "subject".to_string(),
            Self::Subscript => "subscript".to_string(),
            Self::Subscriptions => "subscriptions".to_string(),
            Self::Subtitles => "subtitles".to_string(),
            Self::SubtitlesOff => "subtitles_off".to_string(),
            Self::Subway => "subway".to_string(),
            Self::Summarize => "summarize".to_string(),
            Self::Superscript => "superscript".to_string(),
            Self::SupervisedUserCircle => "supervised_user_circle".to_string(),
            Self::SupervisorAccount => "supervisor_account".to_string(),
            Self::Support => "support".to_string(),
            Self::SupportAgent => "support_agent".to_string(),
            Self::Surfing => "surfing".to_string(),
            Self::SurroundSound => "surround_sound".to_string(),
            Self::SwapCalls => "swap_calls".to_string(),
            Self::SwapHoriz => "swap_horiz".to_string(),
            Self::SwapHorizontalCircle => "swap_horizontal_circle".to_string(),
            Self::SwapVert => "swap_vert".to_string(),
            Self::SwapVertCircle => "swap_vert_circle".to_string(),
            Self::SwapVerticalCircle => "swap_vertical_circle".to_string(),
            Self::Swipe => "swipe".to_string(),
            Self::SwitchAccount => "switch_account".to_string(),
            Self::SwitchCamera => "switch_camera".to_string(),
            Self::SwitchLeft => "switch_left".to_string(),
            Self::SwitchRight => "switch_right".to_string(),
            Self::SwitchVideo => "switch_video".to_string(),
            Self::Sync => "sync".to_string(),
            Self::SyncAlt => "sync_alt".to_string(),
            Self::SyncDisabled => "sync_disabled".to_string(),
            Self::SyncProblem => "sync_problem".to_string(),
            Self::SystemSecurityUpdate => "system_security_update".to_string(),
            Self::SystemSecurityUpdateGood => "system_security_update_good".to_string(),
            Self::SystemSecurityUpdateWarning => {
                "system_security_update_warning".to_string()
            }
            Self::SystemUpdate => "system_update".to_string(),
            Self::SystemUpdateAlt => "system_update_alt".to_string(),
            Self::SystemUpdateTv => "system_update_tv".to_string(),
            Self::Tab => "tab".to_string(),
            Self::TabUnselected => "tab_unselected".to_string(),
            Self::TableChart => "table_chart".to_string(),
            Self::TableRows => "table_rows".to_string(),
            Self::TableView => "table_view".to_string(),
            Self::Tablet => "tablet".to_string(),
            Self::TabletAndroid => "tablet_android".to_string(),
            Self::TabletMac => "tablet_mac".to_string(),
            Self::Tag => "tag".to_string(),
            Self::TagFaces => "tag_faces".to_string(),
            Self::TakeoutDining => "takeout_dining".to_string(),
            Self::TapAndPlay => "tap_and_play".to_string(),
            Self::Tapas => "tapas".to_string(),
            Self::Task => "task".to_string(),
            Self::TaskAlt => "task_alt".to_string(),
            Self::TaxiAlert => "taxi_alert".to_string(),
            Self::TenK => "ten_k".to_string(),
            Self::TenMp => "ten_mp".to_string(),
            Self::Terrain => "terrain".to_string(),
            Self::TextFields => "text_fields".to_string(),
            Self::TextFormat => "text_format".to_string(),
            Self::TextRotateUp => "text_rotate_up".to_string(),
            Self::TextRotateVertical => "text_rotate_vertical".to_string(),
            Self::TextRotationAngledown => "text_rotation_angledown".to_string(),
            Self::TextRotationAngleup => "text_rotation_angleup".to_string(),
            Self::TextRotationDown => "text_rotation_down".to_string(),
            Self::TextRotationNone => "text_rotation_none".to_string(),
            Self::TextSnippet => "text_snippet".to_string(),
            Self::Textsms => "textsms".to_string(),
            Self::Texture => "texture".to_string(),
            Self::TheaterComedy => "theater_comedy".to_string(),
            Self::Theaters => "theaters".to_string(),
            Self::Thermostat => "thermostat".to_string(),
            Self::ThermostatAuto => "thermostat_auto".to_string(),
            Self::ThirteenMp => "thirteen_mp".to_string(),
            Self::ThirtyFps => "thirty_fps".to_string(),
            Self::ThirtyFpsSelect => "thirty_fps_select".to_string(),
            Self::ThreeGMobiledata => "three_g_mobiledata".to_string(),
            Self::ThreeK => "three_k".to_string(),
            Self::ThreeKPlus => "three_k_plus".to_string(),
            Self::ThreeMp => "three_mp".to_string(),
            Self::ThreeP => "three_p".to_string(),
            Self::ThreedRotation => "threed_rotation".to_string(),
            Self::Threesixty => "threesixty".to_string(),
            Self::ThumbDown => "thumb_down".to_string(),
            Self::ThumbDownAlt => "thumb_down_alt".to_string(),
            Self::ThumbDownOffAlt => "thumb_down_off_alt".to_string(),
            Self::ThumbUp => "thumb_up".to_string(),
            Self::ThumbUpAlt => "thumb_up_alt".to_string(),
            Self::ThumbUpOffAlt => "thumb_up_off_alt".to_string(),
            Self::ThumbsUpDown => "thumbs_up_down".to_string(),
            Self::TimeToLeave => "time_to_leave".to_string(),
            Self::Timelapse => "timelapse".to_string(),
            Self::Timeline => "timeline".to_string(),
            Self::Timer => "timer".to_string(),
            Self::Timer10 => "timer_10".to_string(),
            Self::Timer10Select => "timer_10_select".to_string(),
            Self::Timer3 => "timer_3".to_string(),
            Self::Timer3Select => "timer_3_select".to_string(),
            Self::TimerOff => "timer_off".to_string(),
            Self::Title => "title".to_string(),
            Self::Toc => "toc".to_string(),
            Self::Today => "today".to_string(),
            Self::ToggleOff => "toggle_off".to_string(),
            Self::ToggleOn => "toggle_on".to_string(),
            Self::Toll => "toll".to_string(),
            Self::Tonality => "tonality".to_string(),
            Self::Topic => "topic".to_string(),
            Self::TouchApp => "touch_app".to_string(),
            Self::Tour => "tour".to_string(),
            Self::Toys => "toys".to_string(),
            Self::TrackChanges => "track_changes".to_string(),
            Self::Traffic => "traffic".to_string(),
            Self::Train => "train".to_string(),
            Self::Tram => "tram".to_string(),
            Self::TransferWithinAStation => "transfer_within_a_station".to_string(),
            Self::Transform => "transform".to_string(),
            Self::Transgender => "transgender".to_string(),
            Self::TransitEnterexit => "transit_enterexit".to_string(),
            Self::Translate => "translate".to_string(),
            Self::TravelExplore => "travel_explore".to_string(),
            Self::TrendingDown => "trending_down".to_string(),
            Self::TrendingFlat => "trending_flat".to_string(),
            Self::TrendingNeutral => "trending_neutral".to_string(),
            Self::TrendingUp => "trending_up".to_string(),
            Self::TripOrigin => "trip_origin".to_string(),
            Self::TrySmsStar => "try_sms_star".to_string(),
            Self::Tty => "tty".to_string(),
            Self::Tune => "tune".to_string(),
            Self::Tungsten => "tungsten".to_string(),
            Self::TurnedIn => "turned_in".to_string(),
            Self::TurnedInNot => "turned_in_not".to_string(),
            Self::Tv => "tv".to_string(),
            Self::TvOff => "tv_off".to_string(),
            Self::TwelveMp => "twelve_mp".to_string(),
            Self::TwentyFourMp => "twenty_four_mp".to_string(),
            Self::TwentyMp => "twenty_mp".to_string(),
            Self::TwentyOneMp => "twenty_one_mp".to_string(),
            Self::TwentyThreeMp => "twenty_three_mp".to_string(),
            Self::TwentyTwoMp => "twenty_two_mp".to_string(),
            Self::TwoK => "two_k".to_string(),
            Self::TwoKPlus => "two_k_plus".to_string(),
            Self::TwoMp => "two_mp".to_string(),
            Self::TwoWheeler => "two_wheeler".to_string(),
            Self::Umbrella => "umbrella".to_string(),
            Self::Unarchive => "unarchive".to_string(),
            Self::Undo => "undo".to_string(),
            Self::UnfoldLess => "unfold_less".to_string(),
            Self::UnfoldMore => "unfold_more".to_string(),
            Self::Unpublished => "unpublished".to_string(),
            Self::Unsubscribe => "unsubscribe".to_string(),
            Self::Upcoming => "upcoming".to_string(),
            Self::Update => "update".to_string(),
            Self::UpdateDisabled => "update_disabled".to_string(),
            Self::Upgrade => "upgrade".to_string(),
            Self::Upload => "upload".to_string(),
            Self::UploadFile => "upload_file".to_string(),
            Self::Usb => "usb".to_string(),
            Self::UsbOff => "usb_off".to_string(),
            Self::Verified => "verified".to_string(),
            Self::VerifiedUser => "verified_user".to_string(),
            Self::VerticalAlignBottom => "vertical_align_bottom".to_string(),
            Self::VerticalAlignCenter => "vertical_align_center".to_string(),
            Self::VerticalAlignTop => "vertical_align_top".to_string(),
            Self::VerticalDistribute => "vertical_distribute".to_string(),
            Self::VerticalSplit => "vertical_split".to_string(),
            Self::Vibration => "vibration".to_string(),
            Self::VideoCall => "video_call".to_string(),
            Self::VideoCameraBack => "video_camera_back".to_string(),
            Self::VideoCameraFront => "video_camera_front".to_string(),
            Self::VideoCollection => "video_collection".to_string(),
            Self::VideoLabel => "video_label".to_string(),
            Self::VideoLibrary => "video_library".to_string(),
            Self::VideoSettings => "video_settings".to_string(),
            Self::VideoStable => "video_stable".to_string(),
            Self::Videocam => "videocam".to_string(),
            Self::VideocamOff => "videocam_off".to_string(),
            Self::VideogameAsset => "videogame_asset".to_string(),
            Self::VideogameAssetOff => "videogame_asset_off".to_string(),
            Self::ViewAgenda => "view_agenda".to_string(),
            Self::ViewArray => "view_array".to_string(),
            Self::ViewCarousel => "view_carousel".to_string(),
            Self::ViewColumn => "view_column".to_string(),
            Self::ViewComfortable => "view_comfortable".to_string(),
            Self::ViewComfy => "view_comfy".to_string(),
            Self::ViewCompact => "view_compact".to_string(),
            Self::ViewDay => "view_day".to_string(),
            Self::ViewHeadline => "view_headline".to_string(),
            Self::ViewInAr => "view_in_ar".to_string(),
            Self::ViewList => "view_list".to_string(),
            Self::ViewModule => "view_module".to_string(),
            Self::ViewQuilt => "view_quilt".to_string(),
            Self::ViewSidebar => "view_sidebar".to_string(),
            Self::ViewStream => "view_stream".to_string(),
            Self::ViewWeek => "view_week".to_string(),
            Self::Vignette => "vignette".to_string(),
            Self::Villa => "villa".to_string(),
            Self::Visibility => "visibility".to_string(),
            Self::VisibilityOff => "visibility_off".to_string(),
            Self::VoiceChat => "voice_chat".to_string(),
            Self::VoiceOverOff => "voice_over_off".to_string(),
            Self::Voicemail => "voicemail".to_string(),
            Self::VolumeDown => "volume_down".to_string(),
            Self::VolumeMute => "volume_mute".to_string(),
            Self::VolumeOff => "volume_off".to_string(),
            Self::VolumeUp => "volume_up".to_string(),
            Self::VolunteerActivism => "volunteer_activism".to_string(),
            Self::VpnKey => "vpn_key".to_string(),
            Self::VpnLock => "vpn_lock".to_string(),
            Self::Vrpano => "vrpano".to_string(),
            Self::WalletGiftcard => "wallet_giftcard".to_string(),
            Self::WalletMembership => "wallet_membership".to_string(),
            Self::WalletTravel => "wallet_travel".to_string(),
            Self::Wallpaper => "wallpaper".to_string(),
            Self::Warning => "warning".to_string(),
            Self::WarningAmber => "warning_amber".to_string(),
            Self::Wash => "wash".to_string(),
            Self::Watch => "watch".to_string(),
            Self::WatchLater => "watch_later".to_string(),
            Self::Water => "water".to_string(),
            Self::WaterDamage => "water_damage".to_string(),
            Self::WaterfallChart => "waterfall_chart".to_string(),
            Self::Waves => "waves".to_string(),
            Self::WbAuto => "wb_auto".to_string(),
            Self::WbCloudy => "wb_cloudy".to_string(),
            Self::WbIncandescent => "wb_incandescent".to_string(),
            Self::WbIridescent => "wb_iridescent".to_string(),
            Self::WbShade => "wb_shade".to_string(),
            Self::WbSunny => "wb_sunny".to_string(),
            Self::WbTwighlight => "wb_twighlight".to_string(),
            Self::WbTwilight => "wb_twilight".to_string(),
            Self::Wc => "wc".to_string(),
            Self::Web => "web".to_string(),
            Self::WebAsset => "web_asset".to_string(),
            Self::WebAssetOff => "web_asset_off".to_string(),
            Self::WebStories => "web_stories".to_string(),
            Self::Weekend => "weekend".to_string(),
            Self::West => "west".to_string(),
            Self::Whatshot => "whatshot".to_string(),
            Self::WheelchairPickup => "wheelchair_pickup".to_string(),
            Self::WhereToVote => "where_to_vote".to_string(),
            Self::Widgets => "widgets".to_string(),
            Self::Wifi => "wifi".to_string(),
            Self::WifiCalling => "wifi_calling".to_string(),
            Self::WifiCalling3 => "wifi_calling_3".to_string(),
            Self::WifiLock => "wifi_lock".to_string(),
            Self::WifiOff => "wifi_off".to_string(),
            Self::WifiProtectedSetup => "wifi_protected_setup".to_string(),
            Self::WifiTethering => "wifi_tethering".to_string(),
            Self::WifiTetheringOff => "wifi_tethering_off".to_string(),
            Self::Window => "window".to_string(),
            Self::WineBar => "wine_bar".to_string(),
            Self::Work => "work".to_string(),
            Self::WorkOff => "work_off".to_string(),
            Self::WorkOutline => "work_outline".to_string(),
            Self::Workspaces => "workspaces".to_string(),
            Self::WorkspacesFilled => "workspaces_filled".to_string(),
            Self::WorkspacesOutline => "workspaces_outline".to_string(),
            Self::WrapText => "wrap_text".to_string(),
            Self::WrongLocation => "wrong_location".to_string(),
            Self::Wysiwyg => "wysiwyg".to_string(),
            Self::Yard => "yard".to_string(),
            Self::YoutubeSearchedFor => "youtube_searched_for".to_string(),
            Self::ZoomIn => "zoom_in".to_string(),
            Self::ZoomOut => "zoom_out".to_string(),
            Self::ZoomOutMap => "zoom_out_map".to_string(),
            Self::ZoomOutOutlined => "zoom_out_outlined".to_string(),
        }
    }
}
impl std::str::FromStr for StylesIconName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "ac_unit" => Ok(Self::AcUnit),
            "access_alarm" => Ok(Self::AccessAlarm),
            "access_alarms" => Ok(Self::AccessAlarms),
            "access_time" => Ok(Self::AccessTime),
            "access_time_filled" => Ok(Self::AccessTimeFilled),
            "accessibility" => Ok(Self::Accessibility),
            "accessibility_new" => Ok(Self::AccessibilityNew),
            "accessible" => Ok(Self::Accessible),
            "accessible_forward" => Ok(Self::AccessibleForward),
            "account_balance" => Ok(Self::AccountBalance),
            "account_balance_wallet" => Ok(Self::AccountBalanceWallet),
            "account_box" => Ok(Self::AccountBox),
            "account_circle" => Ok(Self::AccountCircle),
            "account_tree" => Ok(Self::AccountTree),
            "ad_units" => Ok(Self::AdUnits),
            "adb" => Ok(Self::Adb),
            "add" => Ok(Self::Add),
            "add_a_photo" => Ok(Self::AddAPhoto),
            "add_alarm" => Ok(Self::AddAlarm),
            "add_alert" => Ok(Self::AddAlert),
            "add_box" => Ok(Self::AddBox),
            "add_business" => Ok(Self::AddBusiness),
            "add_call" => Ok(Self::AddCall),
            "add_chart" => Ok(Self::AddChart),
            "add_circle" => Ok(Self::AddCircle),
            "add_circle_outline" => Ok(Self::AddCircleOutline),
            "add_comment" => Ok(Self::AddComment),
            "add_ic_call" => Ok(Self::AddIcCall),
            "add_link" => Ok(Self::AddLink),
            "add_location" => Ok(Self::AddLocation),
            "add_location_alt" => Ok(Self::AddLocationAlt),
            "add_moderator" => Ok(Self::AddModerator),
            "add_photo_alternate" => Ok(Self::AddPhotoAlternate),
            "add_reaction" => Ok(Self::AddReaction),
            "add_road" => Ok(Self::AddRoad),
            "add_shopping_cart" => Ok(Self::AddShoppingCart),
            "add_task" => Ok(Self::AddTask),
            "add_to_drive" => Ok(Self::AddToDrive),
            "add_to_home_screen" => Ok(Self::AddToHomeScreen),
            "add_to_photos" => Ok(Self::AddToPhotos),
            "add_to_queue" => Ok(Self::AddToQueue),
            "addchart" => Ok(Self::Addchart),
            "adjust" => Ok(Self::Adjust),
            "admin_panel_settings" => Ok(Self::AdminPanelSettings),
            "agriculture" => Ok(Self::Agriculture),
            "air" => Ok(Self::Air),
            "airline_seat_flat" => Ok(Self::AirlineSeatFlat),
            "airline_seat_flat_angled" => Ok(Self::AirlineSeatFlatAngled),
            "airline_seat_individual_suite" => Ok(Self::AirlineSeatIndividualSuite),
            "airline_seat_legroom_extra" => Ok(Self::AirlineSeatLegroomExtra),
            "airline_seat_legroom_normal" => Ok(Self::AirlineSeatLegroomNormal),
            "airline_seat_legroom_reduced" => Ok(Self::AirlineSeatLegroomReduced),
            "airline_seat_recline_extra" => Ok(Self::AirlineSeatReclineExtra),
            "airline_seat_recline_normal" => Ok(Self::AirlineSeatReclineNormal),
            "airplane_ticket" => Ok(Self::AirplaneTicket),
            "airplanemode_active" => Ok(Self::AirplanemodeActive),
            "airplanemode_inactive" => Ok(Self::AirplanemodeInactive),
            "airplanemode_off" => Ok(Self::AirplanemodeOff),
            "airplanemode_on" => Ok(Self::AirplanemodeOn),
            "airplay" => Ok(Self::Airplay),
            "airport_shuttle" => Ok(Self::AirportShuttle),
            "alarm" => Ok(Self::Alarm),
            "alarm_add" => Ok(Self::AlarmAdd),
            "alarm_off" => Ok(Self::AlarmOff),
            "alarm_on" => Ok(Self::AlarmOn),
            "album" => Ok(Self::Album),
            "align_horizontal_center" => Ok(Self::AlignHorizontalCenter),
            "align_horizontal_left" => Ok(Self::AlignHorizontalLeft),
            "align_horizontal_right" => Ok(Self::AlignHorizontalRight),
            "align_vertical_bottom" => Ok(Self::AlignVerticalBottom),
            "align_vertical_center" => Ok(Self::AlignVerticalCenter),
            "align_vertical_top" => Ok(Self::AlignVerticalTop),
            "all_inbox" => Ok(Self::AllInbox),
            "all_inclusive" => Ok(Self::AllInclusive),
            "all_out" => Ok(Self::AllOut),
            "alt_route" => Ok(Self::AltRoute),
            "alternate_email" => Ok(Self::AlternateEmail),
            "amp_stories" => Ok(Self::AmpStories),
            "analytics" => Ok(Self::Analytics),
            "anchor" => Ok(Self::Anchor),
            "android" => Ok(Self::Android),
            "animation" => Ok(Self::Animation),
            "announcement" => Ok(Self::Announcement),
            "aod" => Ok(Self::Aod),
            "apartment" => Ok(Self::Apartment),
            "api" => Ok(Self::Api),
            "app_blocking" => Ok(Self::AppBlocking),
            "app_registration" => Ok(Self::AppRegistration),
            "app_settings_alt" => Ok(Self::AppSettingsAlt),
            "approval" => Ok(Self::Approval),
            "apps" => Ok(Self::Apps),
            "architecture" => Ok(Self::Architecture),
            "archive" => Ok(Self::Archive),
            "arrow_back" => Ok(Self::ArrowBack),
            "arrow_back_ios" => Ok(Self::ArrowBackIos),
            "arrow_back_ios_new" => Ok(Self::ArrowBackIosNew),
            "arrow_circle_down" => Ok(Self::ArrowCircleDown),
            "arrow_circle_up" => Ok(Self::ArrowCircleUp),
            "arrow_downward" => Ok(Self::ArrowDownward),
            "arrow_drop_down" => Ok(Self::ArrowDropDown),
            "arrow_drop_down_circle" => Ok(Self::ArrowDropDownCircle),
            "arrow_drop_up" => Ok(Self::ArrowDropUp),
            "arrow_forward" => Ok(Self::ArrowForward),
            "arrow_forward_ios" => Ok(Self::ArrowForwardIos),
            "arrow_left" => Ok(Self::ArrowLeft),
            "arrow_right" => Ok(Self::ArrowRight),
            "arrow_right_alt" => Ok(Self::ArrowRightAlt),
            "arrow_upward" => Ok(Self::ArrowUpward),
            "art_track" => Ok(Self::ArtTrack),
            "article" => Ok(Self::Article),
            "aspect_ratio" => Ok(Self::AspectRatio),
            "assessment" => Ok(Self::Assessment),
            "assignment" => Ok(Self::Assignment),
            "assignment_ind" => Ok(Self::AssignmentInd),
            "assignment_late" => Ok(Self::AssignmentLate),
            "assignment_return" => Ok(Self::AssignmentReturn),
            "assignment_returned" => Ok(Self::AssignmentReturned),
            "assignment_turned_in" => Ok(Self::AssignmentTurnedIn),
            "assistant" => Ok(Self::Assistant),
            "assistant_direction" => Ok(Self::AssistantDirection),
            "assistant_navigation" => Ok(Self::AssistantNavigation),
            "assistant_photo" => Ok(Self::AssistantPhoto),
            "atm" => Ok(Self::Atm),
            "attach_email" => Ok(Self::AttachEmail),
            "attach_file" => Ok(Self::AttachFile),
            "attach_money" => Ok(Self::AttachMoney),
            "attachment" => Ok(Self::Attachment),
            "attractions" => Ok(Self::Attractions),
            "attribution" => Ok(Self::Attribution),
            "audiotrack" => Ok(Self::Audiotrack),
            "auto_awesome" => Ok(Self::AutoAwesome),
            "auto_awesome_mosaic" => Ok(Self::AutoAwesomeMosaic),
            "auto_awesome_motion" => Ok(Self::AutoAwesomeMotion),
            "auto_delete" => Ok(Self::AutoDelete),
            "auto_fix_high" => Ok(Self::AutoFixHigh),
            "auto_fix_normal" => Ok(Self::AutoFixNormal),
            "auto_fix_off" => Ok(Self::AutoFixOff),
            "auto_graph" => Ok(Self::AutoGraph),
            "auto_stories" => Ok(Self::AutoStories),
            "autofps_select" => Ok(Self::AutofpsSelect),
            "autorenew" => Ok(Self::Autorenew),
            "av_timer" => Ok(Self::AvTimer),
            "baby_changing_station" => Ok(Self::BabyChangingStation),
            "backpack" => Ok(Self::Backpack),
            "backspace" => Ok(Self::Backspace),
            "backup" => Ok(Self::Backup),
            "backup_table" => Ok(Self::BackupTable),
            "badge" => Ok(Self::Badge),
            "bakery_dining" => Ok(Self::BakeryDining),
            "balcony" => Ok(Self::Balcony),
            "ballot" => Ok(Self::Ballot),
            "bar_chart" => Ok(Self::BarChart),
            "batch_prediction" => Ok(Self::BatchPrediction),
            "bathroom" => Ok(Self::Bathroom),
            "bathtub" => Ok(Self::Bathtub),
            "battery_alert" => Ok(Self::BatteryAlert),
            "battery_charging_full" => Ok(Self::BatteryChargingFull),
            "battery_full" => Ok(Self::BatteryFull),
            "battery_saver" => Ok(Self::BatterySaver),
            "battery_std" => Ok(Self::BatteryStd),
            "battery_unknown" => Ok(Self::BatteryUnknown),
            "beach_access" => Ok(Self::BeachAccess),
            "bed" => Ok(Self::Bed),
            "bedroom_baby" => Ok(Self::BedroomBaby),
            "bedroom_child" => Ok(Self::BedroomChild),
            "bedroom_parent" => Ok(Self::BedroomParent),
            "bedtime" => Ok(Self::Bedtime),
            "beenhere" => Ok(Self::Beenhere),
            "bento" => Ok(Self::Bento),
            "bike_scooter" => Ok(Self::BikeScooter),
            "biotech" => Ok(Self::Biotech),
            "blender" => Ok(Self::Blender),
            "block" => Ok(Self::Block),
            "block_flipped" => Ok(Self::BlockFlipped),
            "bloodtype" => Ok(Self::Bloodtype),
            "bluetooth" => Ok(Self::Bluetooth),
            "bluetooth_audio" => Ok(Self::BluetoothAudio),
            "bluetooth_connected" => Ok(Self::BluetoothConnected),
            "bluetooth_disabled" => Ok(Self::BluetoothDisabled),
            "bluetooth_drive" => Ok(Self::BluetoothDrive),
            "bluetooth_searching" => Ok(Self::BluetoothSearching),
            "blur_circular" => Ok(Self::BlurCircular),
            "blur_linear" => Ok(Self::BlurLinear),
            "blur_off" => Ok(Self::BlurOff),
            "blur_on" => Ok(Self::BlurOn),
            "bolt" => Ok(Self::Bolt),
            "book" => Ok(Self::Book),
            "book_online" => Ok(Self::BookOnline),
            "bookmark" => Ok(Self::Bookmark),
            "bookmark_add" => Ok(Self::BookmarkAdd),
            "bookmark_added" => Ok(Self::BookmarkAdded),
            "bookmark_border" => Ok(Self::BookmarkBorder),
            "bookmark_outline" => Ok(Self::BookmarkOutline),
            "bookmark_remove" => Ok(Self::BookmarkRemove),
            "bookmarks" => Ok(Self::Bookmarks),
            "border_all" => Ok(Self::BorderAll),
            "border_bottom" => Ok(Self::BorderBottom),
            "border_clear" => Ok(Self::BorderClear),
            "border_color" => Ok(Self::BorderColor),
            "border_horizontal" => Ok(Self::BorderHorizontal),
            "border_inner" => Ok(Self::BorderInner),
            "border_left" => Ok(Self::BorderLeft),
            "border_outer" => Ok(Self::BorderOuter),
            "border_right" => Ok(Self::BorderRight),
            "border_style" => Ok(Self::BorderStyle),
            "border_top" => Ok(Self::BorderTop),
            "border_vertical" => Ok(Self::BorderVertical),
            "branding_watermark" => Ok(Self::BrandingWatermark),
            "breakfast_dining" => Ok(Self::BreakfastDining),
            "brightness_1" => Ok(Self::Brightness1),
            "brightness_2" => Ok(Self::Brightness2),
            "brightness_3" => Ok(Self::Brightness3),
            "brightness_4" => Ok(Self::Brightness4),
            "brightness_5" => Ok(Self::Brightness5),
            "brightness_6" => Ok(Self::Brightness6),
            "brightness_7" => Ok(Self::Brightness7),
            "brightness_auto" => Ok(Self::BrightnessAuto),
            "brightness_high" => Ok(Self::BrightnessHigh),
            "brightness_low" => Ok(Self::BrightnessLow),
            "brightness_medium" => Ok(Self::BrightnessMedium),
            "broken_image" => Ok(Self::BrokenImage),
            "browser_not_supported" => Ok(Self::BrowserNotSupported),
            "brunch_dining" => Ok(Self::BrunchDining),
            "brush" => Ok(Self::Brush),
            "bubble_chart" => Ok(Self::BubbleChart),
            "bug_report" => Ok(Self::BugReport),
            "build" => Ok(Self::Build),
            "build_circle" => Ok(Self::BuildCircle),
            "bungalow" => Ok(Self::Bungalow),
            "burst_mode" => Ok(Self::BurstMode),
            "bus_alert" => Ok(Self::BusAlert),
            "business" => Ok(Self::Business),
            "business_center" => Ok(Self::BusinessCenter),
            "cabin" => Ok(Self::Cabin),
            "cable" => Ok(Self::Cable),
            "cached" => Ok(Self::Cached),
            "cake" => Ok(Self::Cake),
            "calculate" => Ok(Self::Calculate),
            "calendar_today" => Ok(Self::CalendarToday),
            "calendar_view_day" => Ok(Self::CalendarViewDay),
            "calendar_view_month" => Ok(Self::CalendarViewMonth),
            "calendar_view_week" => Ok(Self::CalendarViewWeek),
            "call" => Ok(Self::Call),
            "call_end" => Ok(Self::CallEnd),
            "call_made" => Ok(Self::CallMade),
            "call_merge" => Ok(Self::CallMerge),
            "call_missed" => Ok(Self::CallMissed),
            "call_missed_outgoing" => Ok(Self::CallMissedOutgoing),
            "call_received" => Ok(Self::CallReceived),
            "call_split" => Ok(Self::CallSplit),
            "call_to_action" => Ok(Self::CallToAction),
            "camera" => Ok(Self::Camera),
            "camera_alt" => Ok(Self::CameraAlt),
            "camera_enhance" => Ok(Self::CameraEnhance),
            "camera_front" => Ok(Self::CameraFront),
            "camera_indoor" => Ok(Self::CameraIndoor),
            "camera_outdoor" => Ok(Self::CameraOutdoor),
            "camera_rear" => Ok(Self::CameraRear),
            "camera_roll" => Ok(Self::CameraRoll),
            "cameraswitch" => Ok(Self::Cameraswitch),
            "campaign" => Ok(Self::Campaign),
            "cancel" => Ok(Self::Cancel),
            "cancel_presentation" => Ok(Self::CancelPresentation),
            "cancel_schedule_send" => Ok(Self::CancelScheduleSend),
            "car_rental" => Ok(Self::CarRental),
            "car_repair" => Ok(Self::CarRepair),
            "card_giftcard" => Ok(Self::CardGiftcard),
            "card_membership" => Ok(Self::CardMembership),
            "card_travel" => Ok(Self::CardTravel),
            "carpenter" => Ok(Self::Carpenter),
            "cases" => Ok(Self::Cases),
            "casino" => Ok(Self::Casino),
            "cast" => Ok(Self::Cast),
            "cast_connected" => Ok(Self::CastConnected),
            "cast_for_education" => Ok(Self::CastForEducation),
            "catching_pokemon" => Ok(Self::CatchingPokemon),
            "category" => Ok(Self::Category),
            "celebration" => Ok(Self::Celebration),
            "cell_wifi" => Ok(Self::CellWifi),
            "center_focus_strong" => Ok(Self::CenterFocusStrong),
            "center_focus_weak" => Ok(Self::CenterFocusWeak),
            "chair" => Ok(Self::Chair),
            "chair_alt" => Ok(Self::ChairAlt),
            "chalet" => Ok(Self::Chalet),
            "change_circle" => Ok(Self::ChangeCircle),
            "change_history" => Ok(Self::ChangeHistory),
            "charging_station" => Ok(Self::ChargingStation),
            "chat" => Ok(Self::Chat),
            "chat_bubble" => Ok(Self::ChatBubble),
            "chat_bubble_outline" => Ok(Self::ChatBubbleOutline),
            "check" => Ok(Self::Check),
            "check_box" => Ok(Self::CheckBox),
            "check_box_outline_blank" => Ok(Self::CheckBoxOutlineBlank),
            "check_circle" => Ok(Self::CheckCircle),
            "check_circle_outline" => Ok(Self::CheckCircleOutline),
            "checklist" => Ok(Self::Checklist),
            "checklist_rtl" => Ok(Self::ChecklistRtl),
            "checkroom" => Ok(Self::Checkroom),
            "chevron_left" => Ok(Self::ChevronLeft),
            "chevron_right" => Ok(Self::ChevronRight),
            "child_care" => Ok(Self::ChildCare),
            "child_friendly" => Ok(Self::ChildFriendly),
            "chrome_reader_mode" => Ok(Self::ChromeReaderMode),
            "circle" => Ok(Self::Circle),
            "circle_notifications" => Ok(Self::CircleNotifications),
            "class_" => Ok(Self::Class),
            "clean_hands" => Ok(Self::CleanHands),
            "cleaning_services" => Ok(Self::CleaningServices),
            "clear" => Ok(Self::Clear),
            "clear_all" => Ok(Self::ClearAll),
            "close" => Ok(Self::Close),
            "close_fullscreen" => Ok(Self::CloseFullscreen),
            "closed_caption" => Ok(Self::ClosedCaption),
            "closed_caption_disabled" => Ok(Self::ClosedCaptionDisabled),
            "closed_caption_off" => Ok(Self::ClosedCaptionOff),
            "cloud" => Ok(Self::Cloud),
            "cloud_circle" => Ok(Self::CloudCircle),
            "cloud_done" => Ok(Self::CloudDone),
            "cloud_download" => Ok(Self::CloudDownload),
            "cloud_off" => Ok(Self::CloudOff),
            "cloud_queue" => Ok(Self::CloudQueue),
            "cloud_upload" => Ok(Self::CloudUpload),
            "code" => Ok(Self::Code),
            "code_off" => Ok(Self::CodeOff),
            "coffee" => Ok(Self::Coffee),
            "coffee_maker" => Ok(Self::CoffeeMaker),
            "collections" => Ok(Self::Collections),
            "collections_bookmark" => Ok(Self::CollectionsBookmark),
            "color_lens" => Ok(Self::ColorLens),
            "colorize" => Ok(Self::Colorize),
            "comment" => Ok(Self::Comment),
            "comment_bank" => Ok(Self::CommentBank),
            "commute" => Ok(Self::Commute),
            "compare" => Ok(Self::Compare),
            "compare_arrows" => Ok(Self::CompareArrows),
            "compass_calibration" => Ok(Self::CompassCalibration),
            "compress" => Ok(Self::Compress),
            "computer" => Ok(Self::Computer),
            "confirmation_num" => Ok(Self::ConfirmationNum),
            "confirmation_number" => Ok(Self::ConfirmationNumber),
            "connect_without_contact" => Ok(Self::ConnectWithoutContact),
            "connected_tv" => Ok(Self::ConnectedTv),
            "construction" => Ok(Self::Construction),
            "contact_mail" => Ok(Self::ContactMail),
            "contact_page" => Ok(Self::ContactPage),
            "contact_phone" => Ok(Self::ContactPhone),
            "contact_support" => Ok(Self::ContactSupport),
            "contactless" => Ok(Self::Contactless),
            "contacts" => Ok(Self::Contacts),
            "content_copy" => Ok(Self::ContentCopy),
            "content_cut" => Ok(Self::ContentCut),
            "content_paste" => Ok(Self::ContentPaste),
            "content_paste_off" => Ok(Self::ContentPasteOff),
            "control_camera" => Ok(Self::ControlCamera),
            "control_point" => Ok(Self::ControlPoint),
            "control_point_duplicate" => Ok(Self::ControlPointDuplicate),
            "copy" => Ok(Self::Copy),
            "copy_all" => Ok(Self::CopyAll),
            "copyright" => Ok(Self::Copyright),
            "coronavirus" => Ok(Self::Coronavirus),
            "corporate_fare" => Ok(Self::CorporateFare),
            "cottage" => Ok(Self::Cottage),
            "countertops" => Ok(Self::Countertops),
            "create" => Ok(Self::Create),
            "create_new_folder" => Ok(Self::CreateNewFolder),
            "credit_card" => Ok(Self::CreditCard),
            "credit_card_off" => Ok(Self::CreditCardOff),
            "credit_score" => Ok(Self::CreditScore),
            "crib" => Ok(Self::Crib),
            "crop" => Ok(Self::Crop),
            "crop_16_9" => Ok(Self::Crop169),
            "crop_3_2" => Ok(Self::Crop32),
            "crop_5_4" => Ok(Self::Crop54),
            "crop_7_5" => Ok(Self::Crop75),
            "crop_din" => Ok(Self::CropDin),
            "crop_free" => Ok(Self::CropFree),
            "crop_landscape" => Ok(Self::CropLandscape),
            "crop_original" => Ok(Self::CropOriginal),
            "crop_portrait" => Ok(Self::CropPortrait),
            "crop_rotate" => Ok(Self::CropRotate),
            "crop_square" => Ok(Self::CropSquare),
            "cut" => Ok(Self::Cut),
            "dangerous" => Ok(Self::Dangerous),
            "dark_mode" => Ok(Self::DarkMode),
            "dashboard" => Ok(Self::Dashboard),
            "dashboard_customize" => Ok(Self::DashboardCustomize),
            "data_saver_off" => Ok(Self::DataSaverOff),
            "data_saver_on" => Ok(Self::DataSaverOn),
            "data_usage" => Ok(Self::DataUsage),
            "date_range" => Ok(Self::DateRange),
            "deck" => Ok(Self::Deck),
            "dehaze" => Ok(Self::Dehaze),
            "delete" => Ok(Self::Delete),
            "delete_forever" => Ok(Self::DeleteForever),
            "delete_outline" => Ok(Self::DeleteOutline),
            "delete_sweep" => Ok(Self::DeleteSweep),
            "delivery_dining" => Ok(Self::DeliveryDining),
            "departure_board" => Ok(Self::DepartureBoard),
            "description" => Ok(Self::Description),
            "design_services" => Ok(Self::DesignServices),
            "desktop_access_disabled" => Ok(Self::DesktopAccessDisabled),
            "desktop_mac" => Ok(Self::DesktopMac),
            "desktop_windows" => Ok(Self::DesktopWindows),
            "details" => Ok(Self::Details),
            "developer_board" => Ok(Self::DeveloperBoard),
            "developer_board_off" => Ok(Self::DeveloperBoardOff),
            "developer_mode" => Ok(Self::DeveloperMode),
            "device_hub" => Ok(Self::DeviceHub),
            "device_thermostat" => Ok(Self::DeviceThermostat),
            "device_unknown" => Ok(Self::DeviceUnknown),
            "devices" => Ok(Self::Devices),
            "devices_other" => Ok(Self::DevicesOther),
            "dialer_sip" => Ok(Self::DialerSip),
            "dialpad" => Ok(Self::Dialpad),
            "dining" => Ok(Self::Dining),
            "dinner_dining" => Ok(Self::DinnerDining),
            "directions" => Ok(Self::Directions),
            "directions_bike" => Ok(Self::DirectionsBike),
            "directions_boat" => Ok(Self::DirectionsBoat),
            "directions_boat_filled" => Ok(Self::DirectionsBoatFilled),
            "directions_bus" => Ok(Self::DirectionsBus),
            "directions_bus_filled" => Ok(Self::DirectionsBusFilled),
            "directions_car" => Ok(Self::DirectionsCar),
            "directions_car_filled" => Ok(Self::DirectionsCarFilled),
            "directions_ferry" => Ok(Self::DirectionsFerry),
            "directions_off" => Ok(Self::DirectionsOff),
            "directions_railway_filled" => Ok(Self::DirectionsRailwayFilled),
            "directions_run" => Ok(Self::DirectionsRun),
            "directions_railway" => Ok(Self::DirectionsRailway),
            "directions_subway" => Ok(Self::DirectionsSubway),
            "directions_subway_filled" => Ok(Self::DirectionsSubwayFilled),
            "directions_train" => Ok(Self::DirectionsTrain),
            "directions_transit" => Ok(Self::DirectionsTransit),
            "directions_transit_filled" => Ok(Self::DirectionsTransitFilled),
            "directions_walk" => Ok(Self::DirectionsWalk),
            "dirty_lens" => Ok(Self::DirtyLens),
            "disabled_by_default" => Ok(Self::DisabledByDefault),
            "disc_full" => Ok(Self::DiscFull),
            "dnd_forwardslash" => Ok(Self::DndForwardslash),
            "dns" => Ok(Self::Dns),
            "do_disturb" => Ok(Self::DoDisturb),
            "do_disturb_alt" => Ok(Self::DoDisturbAlt),
            "do_disturb_off" => Ok(Self::DoDisturbOff),
            "do_disturb_on" => Ok(Self::DoDisturbOn),
            "do_not_disturb" => Ok(Self::DoNotDisturb),
            "do_not_disturb_alt" => Ok(Self::DoNotDisturbAlt),
            "do_not_disturb_off" => Ok(Self::DoNotDisturbOff),
            "do_not_disturb_on" => Ok(Self::DoNotDisturbOn),
            "do_not_disturb_on_total_silence" => Ok(Self::DoNotDisturbOnTotalSilence),
            "do_not_step" => Ok(Self::DoNotStep),
            "do_not_touch" => Ok(Self::DoNotTouch),
            "dock" => Ok(Self::Dock),
            "document_scanner" => Ok(Self::DocumentScanner),
            "domain" => Ok(Self::Domain),
            "domain_disabled" => Ok(Self::DomainDisabled),
            "domain_verification" => Ok(Self::DomainVerification),
            "done" => Ok(Self::Done),
            "done_all" => Ok(Self::DoneAll),
            "done_outline" => Ok(Self::DoneOutline),
            "donut_large" => Ok(Self::DonutLarge),
            "donut_small" => Ok(Self::DonutSmall),
            "door_back" => Ok(Self::DoorBack),
            "door_front" => Ok(Self::DoorFront),
            "door_sliding" => Ok(Self::DoorSliding),
            "doorbell" => Ok(Self::Doorbell),
            "double_arrow" => Ok(Self::DoubleArrow),
            "downhill_skiing" => Ok(Self::DownhillSkiing),
            "download" => Ok(Self::Download),
            "download_done" => Ok(Self::DownloadDone),
            "download_for_offline" => Ok(Self::DownloadForOffline),
            "downloading" => Ok(Self::Downloading),
            "drafts" => Ok(Self::Drafts),
            "drag_handle" => Ok(Self::DragHandle),
            "drag_indicator" => Ok(Self::DragIndicator),
            "drive_eta" => Ok(Self::DriveEta),
            "drive_file_move" => Ok(Self::DriveFileMove),
            "drive_file_move_outline" => Ok(Self::DriveFileMoveOutline),
            "drive_file_rename_outline" => Ok(Self::DriveFileRenameOutline),
            "drive_folder_upload" => Ok(Self::DriveFolderUpload),
            "dry" => Ok(Self::Dry),
            "dry_cleaning" => Ok(Self::DryCleaning),
            "duo" => Ok(Self::Duo),
            "dvr" => Ok(Self::Dvr),
            "dynamic_feed" => Ok(Self::DynamicFeed),
            "dynamic_form" => Ok(Self::DynamicForm),
            "e_mobiledata" => Ok(Self::EMobiledata),
            "earbuds" => Ok(Self::Earbuds),
            "earbuds_battery" => Ok(Self::EarbudsBattery),
            "east" => Ok(Self::East),
            "eco" => Ok(Self::Eco),
            "edgesensor_high" => Ok(Self::EdgesensorHigh),
            "edgesensor_low" => Ok(Self::EdgesensorLow),
            "edit" => Ok(Self::Edit),
            "edit_attributes" => Ok(Self::EditAttributes),
            "edit_location" => Ok(Self::EditLocation),
            "edit_location_alt" => Ok(Self::EditLocationAlt),
            "edit_notifications" => Ok(Self::EditNotifications),
            "edit_off" => Ok(Self::EditOff),
            "edit_road" => Ok(Self::EditRoad),
            "eight_k" => Ok(Self::EightK),
            "eight_k_plus" => Ok(Self::EightKPlus),
            "eight_mp" => Ok(Self::EightMp),
            "eighteen_mp" => Ok(Self::EighteenMp),
            "eject" => Ok(Self::Eject),
            "elderly" => Ok(Self::Elderly),
            "electric_bike" => Ok(Self::ElectricBike),
            "electric_car" => Ok(Self::ElectricCar),
            "electric_moped" => Ok(Self::ElectricMoped),
            "electric_rickshaw" => Ok(Self::ElectricRickshaw),
            "electric_scooter" => Ok(Self::ElectricScooter),
            "electrical_services" => Ok(Self::ElectricalServices),
            "elevator" => Ok(Self::Elevator),
            "eleven_mp" => Ok(Self::ElevenMp),
            "email" => Ok(Self::Email),
            "emoji_emotions" => Ok(Self::EmojiEmotions),
            "emoji_events" => Ok(Self::EmojiEvents),
            "emoji_flags" => Ok(Self::EmojiFlags),
            "emoji_food_beverage" => Ok(Self::EmojiFoodBeverage),
            "emoji_nature" => Ok(Self::EmojiNature),
            "emoji_objects" => Ok(Self::EmojiObjects),
            "emoji_people" => Ok(Self::EmojiPeople),
            "emoji_symbols" => Ok(Self::EmojiSymbols),
            "emoji_transportation" => Ok(Self::EmojiTransportation),
            "engineering" => Ok(Self::Engineering),
            "enhance_photo_translate" => Ok(Self::EnhancePhotoTranslate),
            "enhanced_encryption" => Ok(Self::EnhancedEncryption),
            "equalizer" => Ok(Self::Equalizer),
            "error" => Ok(Self::Error),
            "error_outline" => Ok(Self::ErrorOutline),
            "escalator" => Ok(Self::Escalator),
            "escalator_warning" => Ok(Self::EscalatorWarning),
            "euro" => Ok(Self::Euro),
            "euro_symbol" => Ok(Self::EuroSymbol),
            "ev_station" => Ok(Self::EvStation),
            "event" => Ok(Self::Event),
            "event_available" => Ok(Self::EventAvailable),
            "event_busy" => Ok(Self::EventBusy),
            "event_note" => Ok(Self::EventNote),
            "event_seat" => Ok(Self::EventSeat),
            "exit_to_app" => Ok(Self::ExitToApp),
            "expand" => Ok(Self::Expand),
            "expand_less" => Ok(Self::ExpandLess),
            "expand_more" => Ok(Self::ExpandMore),
            "explicit" => Ok(Self::Explicit),
            "explore" => Ok(Self::Explore),
            "explore_off" => Ok(Self::ExploreOff),
            "exposure" => Ok(Self::Exposure),
            "exposure_minus_1" => Ok(Self::ExposureMinus1),
            "exposure_minus_2" => Ok(Self::ExposureMinus2),
            "exposure_neg_1" => Ok(Self::ExposureNeg1),
            "exposure_neg_2" => Ok(Self::ExposureNeg2),
            "exposure_plus_1" => Ok(Self::ExposurePlus1),
            "exposure_plus_2" => Ok(Self::ExposurePlus2),
            "exposure_zero" => Ok(Self::ExposureZero),
            "extension" => Ok(Self::Extension),
            "extension_off" => Ok(Self::ExtensionOff),
            "face" => Ok(Self::Face),
            "face_retouching_off" => Ok(Self::FaceRetouchingOff),
            "face_retouching_natural" => Ok(Self::FaceRetouchingNatural),
            "facebook" => Ok(Self::Facebook),
            "fact_check" => Ok(Self::FactCheck),
            "family_restroom" => Ok(Self::FamilyRestroom),
            "fast_forward" => Ok(Self::FastForward),
            "fast_rewind" => Ok(Self::FastRewind),
            "fastfood" => Ok(Self::Fastfood),
            "favorite" => Ok(Self::Favorite),
            "favorite_border" => Ok(Self::FavoriteBorder),
            "favorite_outline" => Ok(Self::FavoriteOutline),
            "featured_play_list" => Ok(Self::FeaturedPlayList),
            "featured_video" => Ok(Self::FeaturedVideo),
            "feed" => Ok(Self::Feed),
            "feedback" => Ok(Self::Feedback),
            "female" => Ok(Self::Female),
            "fence" => Ok(Self::Fence),
            "festival" => Ok(Self::Festival),
            "fiber_dvr" => Ok(Self::FiberDvr),
            "fiber_manual_record" => Ok(Self::FiberManualRecord),
            "fiber_new" => Ok(Self::FiberNew),
            "fiber_pin" => Ok(Self::FiberPin),
            "fiber_smart_record" => Ok(Self::FiberSmartRecord),
            "fifteen_mp" => Ok(Self::FifteenMp),
            "file_copy" => Ok(Self::FileCopy),
            "file_download" => Ok(Self::FileDownload),
            "file_download_done" => Ok(Self::FileDownloadDone),
            "file_download_off" => Ok(Self::FileDownloadOff),
            "file_present" => Ok(Self::FilePresent),
            "file_upload" => Ok(Self::FileUpload),
            "filter" => Ok(Self::Filter),
            "filter_1" => Ok(Self::Filter1),
            "filter_2" => Ok(Self::Filter2),
            "filter_3" => Ok(Self::Filter3),
            "filter_4" => Ok(Self::Filter4),
            "filter_5" => Ok(Self::Filter5),
            "filter_6" => Ok(Self::Filter6),
            "filter_7" => Ok(Self::Filter7),
            "filter_8" => Ok(Self::Filter8),
            "filter_9" => Ok(Self::Filter9),
            "filter_9_plus" => Ok(Self::Filter9Plus),
            "filter_alt" => Ok(Self::FilterAlt),
            "filter_b_and_w" => Ok(Self::FilterBAndW),
            "filter_center_focus" => Ok(Self::FilterCenterFocus),
            "filter_drama" => Ok(Self::FilterDrama),
            "filter_frames" => Ok(Self::FilterFrames),
            "filter_hdr" => Ok(Self::FilterHdr),
            "filter_list" => Ok(Self::FilterList),
            "filter_list_alt" => Ok(Self::FilterListAlt),
            "filter_none" => Ok(Self::FilterNone),
            "filter_tilt_shift" => Ok(Self::FilterTiltShift),
            "filter_vintage" => Ok(Self::FilterVintage),
            "find_in_page" => Ok(Self::FindInPage),
            "find_replace" => Ok(Self::FindReplace),
            "fingerprint" => Ok(Self::Fingerprint),
            "fire_extinguisher" => Ok(Self::FireExtinguisher),
            "fire_hydrant" => Ok(Self::FireHydrant),
            "fireplace" => Ok(Self::Fireplace),
            "first_page" => Ok(Self::FirstPage),
            "fit_screen" => Ok(Self::FitScreen),
            "fitness_center" => Ok(Self::FitnessCenter),
            "five_g" => Ok(Self::FiveG),
            "five_k" => Ok(Self::FiveK),
            "five_k_plus" => Ok(Self::FiveKPlus),
            "five_mp" => Ok(Self::FiveMp),
            "flag" => Ok(Self::Flag),
            "flaky" => Ok(Self::Flaky),
            "flare" => Ok(Self::Flare),
            "flash_auto" => Ok(Self::FlashAuto),
            "flash_off" => Ok(Self::FlashOff),
            "flash_on" => Ok(Self::FlashOn),
            "flashlight_off" => Ok(Self::FlashlightOff),
            "flashlight_on" => Ok(Self::FlashlightOn),
            "flatware" => Ok(Self::Flatware),
            "flight" => Ok(Self::Flight),
            "flight_land" => Ok(Self::FlightLand),
            "flight_takeoff" => Ok(Self::FlightTakeoff),
            "flip" => Ok(Self::Flip),
            "flip_camera_android" => Ok(Self::FlipCameraAndroid),
            "flip_camera_ios" => Ok(Self::FlipCameraIos),
            "flip_to_back" => Ok(Self::FlipToBack),
            "flip_to_front" => Ok(Self::FlipToFront),
            "flourescent" => Ok(Self::Flourescent),
            "flutter_dash" => Ok(Self::FlutterDash),
            "fmd_bad" => Ok(Self::FmdBad),
            "fmd_good" => Ok(Self::FmdGood),
            "folder" => Ok(Self::Folder),
            "folder_open" => Ok(Self::FolderOpen),
            "folder_shared" => Ok(Self::FolderShared),
            "folder_special" => Ok(Self::FolderSpecial),
            "follow_the_signs" => Ok(Self::FollowTheSigns),
            "font_download" => Ok(Self::FontDownload),
            "font_download_off" => Ok(Self::FontDownloadOff),
            "food_bank" => Ok(Self::FoodBank),
            "format_align_center" => Ok(Self::FormatAlignCenter),
            "format_align_justify" => Ok(Self::FormatAlignJustify),
            "format_align_left" => Ok(Self::FormatAlignLeft),
            "format_align_right" => Ok(Self::FormatAlignRight),
            "format_bold" => Ok(Self::FormatBold),
            "format_clear" => Ok(Self::FormatClear),
            "format_color_fill" => Ok(Self::FormatColorFill),
            "format_color_reset" => Ok(Self::FormatColorReset),
            "format_color_text" => Ok(Self::FormatColorText),
            "format_indent_decrease" => Ok(Self::FormatIndentDecrease),
            "format_indent_increase" => Ok(Self::FormatIndentIncrease),
            "format_italic" => Ok(Self::FormatItalic),
            "format_line_spacing" => Ok(Self::FormatLineSpacing),
            "format_list_bulleted" => Ok(Self::FormatListBulleted),
            "format_list_numbered" => Ok(Self::FormatListNumbered),
            "format_list_numbered_rtl" => Ok(Self::FormatListNumberedRtl),
            "format_paint" => Ok(Self::FormatPaint),
            "format_quote" => Ok(Self::FormatQuote),
            "format_shapes" => Ok(Self::FormatShapes),
            "format_size" => Ok(Self::FormatSize),
            "format_strikethrough" => Ok(Self::FormatStrikethrough),
            "format_textdirection_l_to_r" => Ok(Self::FormatTextdirectionLToR),
            "format_textdirection_r_to_l" => Ok(Self::FormatTextdirectionRToL),
            "format_underline" => Ok(Self::FormatUnderline),
            "format_underlined" => Ok(Self::FormatUnderlined),
            "forum" => Ok(Self::Forum),
            "forward" => Ok(Self::Forward),
            "forward_10" => Ok(Self::Forward10),
            "forward_30" => Ok(Self::Forward30),
            "forward_5" => Ok(Self::Forward5),
            "forward_to_inbox" => Ok(Self::ForwardToInbox),
            "foundation" => Ok(Self::Foundation),
            "four_g_mobiledata" => Ok(Self::FourGMobiledata),
            "four_g_plus_mobiledata" => Ok(Self::FourGPlusMobiledata),
            "four_k" => Ok(Self::FourK),
            "four_k_plus" => Ok(Self::FourKPlus),
            "four_mp" => Ok(Self::FourMp),
            "fourteen_mp" => Ok(Self::FourteenMp),
            "free_breakfast" => Ok(Self::FreeBreakfast),
            "fullscreen" => Ok(Self::Fullscreen),
            "fullscreen_exit" => Ok(Self::FullscreenExit),
            "functions" => Ok(Self::Functions),
            "g_mobiledata" => Ok(Self::GMobiledata),
            "g_translate" => Ok(Self::GTranslate),
            "gamepad" => Ok(Self::Gamepad),
            "games" => Ok(Self::Games),
            "garage" => Ok(Self::Garage),
            "gavel" => Ok(Self::Gavel),
            "gesture" => Ok(Self::Gesture),
            "get_app" => Ok(Self::GetApp),
            "gif" => Ok(Self::Gif),
            "gite" => Ok(Self::Gite),
            "golf_course" => Ok(Self::GolfCourse),
            "gpp_bad" => Ok(Self::GppBad),
            "gpp_good" => Ok(Self::GppGood),
            "gpp_maybe" => Ok(Self::GppMaybe),
            "gps_fixed" => Ok(Self::GpsFixed),
            "gps_not_fixed" => Ok(Self::GpsNotFixed),
            "gps_off" => Ok(Self::GpsOff),
            "grade" => Ok(Self::Grade),
            "gradient" => Ok(Self::Gradient),
            "grading" => Ok(Self::Grading),
            "grain" => Ok(Self::Grain),
            "graphic_eq" => Ok(Self::GraphicEq),
            "grass" => Ok(Self::Grass),
            "grid_3x3" => Ok(Self::Grid3x3),
            "grid_4x4" => Ok(Self::Grid4x4),
            "grid_goldenratio" => Ok(Self::GridGoldenratio),
            "grid_off" => Ok(Self::GridOff),
            "grid_on" => Ok(Self::GridOn),
            "grid_view" => Ok(Self::GridView),
            "group" => Ok(Self::Group),
            "group_add" => Ok(Self::GroupAdd),
            "group_work" => Ok(Self::GroupWork),
            "groups" => Ok(Self::Groups),
            "h_mobiledata" => Ok(Self::HMobiledata),
            "h_plus_mobiledata" => Ok(Self::HPlusMobiledata),
            "hail" => Ok(Self::Hail),
            "handyman" => Ok(Self::Handyman),
            "hardware" => Ok(Self::Hardware),
            "hd" => Ok(Self::Hd),
            "hdr_auto" => Ok(Self::HdrAuto),
            "hdr_auto_select" => Ok(Self::HdrAutoSelect),
            "hdr_enhanced_select" => Ok(Self::HdrEnhancedSelect),
            "hdr_off" => Ok(Self::HdrOff),
            "hdr_off_select" => Ok(Self::HdrOffSelect),
            "hdr_on" => Ok(Self::HdrOn),
            "hdr_on_select" => Ok(Self::HdrOnSelect),
            "hdr_plus" => Ok(Self::HdrPlus),
            "hdr_strong" => Ok(Self::HdrStrong),
            "hdr_weak" => Ok(Self::HdrWeak),
            "headphones" => Ok(Self::Headphones),
            "headphones_battery" => Ok(Self::HeadphonesBattery),
            "headset" => Ok(Self::Headset),
            "headset_mic" => Ok(Self::HeadsetMic),
            "headset_off" => Ok(Self::HeadsetOff),
            "healing" => Ok(Self::Healing),
            "health_and_safety" => Ok(Self::HealthAndSafety),
            "hearing" => Ok(Self::Hearing),
            "hearing_disabled" => Ok(Self::HearingDisabled),
            "height" => Ok(Self::Height),
            "help" => Ok(Self::Help),
            "help_center" => Ok(Self::HelpCenter),
            "help_outline" => Ok(Self::HelpOutline),
            "hevc" => Ok(Self::Hevc),
            "hide_image" => Ok(Self::HideImage),
            "hide_source" => Ok(Self::HideSource),
            "high_quality" => Ok(Self::HighQuality),
            "highlight" => Ok(Self::Highlight),
            "highlight_alt" => Ok(Self::HighlightAlt),
            "highlight_off" => Ok(Self::HighlightOff),
            "highlight_remove" => Ok(Self::HighlightRemove),
            "hiking" => Ok(Self::Hiking),
            "history" => Ok(Self::History),
            "history_edu" => Ok(Self::HistoryEdu),
            "history_toggle_off" => Ok(Self::HistoryToggleOff),
            "holiday_village" => Ok(Self::HolidayVillage),
            "home" => Ok(Self::Home),
            "home_filled" => Ok(Self::HomeFilled),
            "home_max" => Ok(Self::HomeMax),
            "home_mini" => Ok(Self::HomeMini),
            "home_repair_service" => Ok(Self::HomeRepairService),
            "home_work" => Ok(Self::HomeWork),
            "horizontal_distribute" => Ok(Self::HorizontalDistribute),
            "horizontal_rule" => Ok(Self::HorizontalRule),
            "horizontal_split" => Ok(Self::HorizontalSplit),
            "hot_tub" => Ok(Self::HotTub),
            "hotel" => Ok(Self::Hotel),
            "hourglass_bottom" => Ok(Self::HourglassBottom),
            "hourglass_disabled" => Ok(Self::HourglassDisabled),
            "hourglass_empty" => Ok(Self::HourglassEmpty),
            "hourglass_full" => Ok(Self::HourglassFull),
            "hourglass_top" => Ok(Self::HourglassTop),
            "house" => Ok(Self::House),
            "house_siding" => Ok(Self::HouseSiding),
            "houseboat" => Ok(Self::Houseboat),
            "how_to_reg" => Ok(Self::HowToReg),
            "how_to_vote" => Ok(Self::HowToVote),
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            "hvac" => Ok(Self::Hvac),
            "ice_skating" => Ok(Self::IceSkating),
            "icecream" => Ok(Self::Icecream),
            "image" => Ok(Self::Image),
            "image_aspect_ratio" => Ok(Self::ImageAspectRatio),
            "image_not_supported" => Ok(Self::ImageNotSupported),
            "image_search" => Ok(Self::ImageSearch),
            "imagesearch_roller" => Ok(Self::ImagesearchRoller),
            "import_contacts" => Ok(Self::ImportContacts),
            "import_export" => Ok(Self::ImportExport),
            "important_devices" => Ok(Self::ImportantDevices),
            "inbox" => Ok(Self::Inbox),
            "indeterminate_check_box" => Ok(Self::IndeterminateCheckBox),
            "info" => Ok(Self::Info),
            "info_outline" => Ok(Self::InfoOutline),
            "input" => Ok(Self::Input),
            "insert_chart" => Ok(Self::InsertChart),
            "insert_comment" => Ok(Self::InsertComment),
            "insert_drive_file" => Ok(Self::InsertDriveFile),
            "insert_emoticon" => Ok(Self::InsertEmoticon),
            "insert_invitation" => Ok(Self::InsertInvitation),
            "insert_link" => Ok(Self::InsertLink),
            "insert_photo" => Ok(Self::InsertPhoto),
            "insights" => Ok(Self::Insights),
            "integration_instructions" => Ok(Self::IntegrationInstructions),
            "inventory" => Ok(Self::Inventory),
            "inventory_2" => Ok(Self::Inventory2),
            "invert_colors" => Ok(Self::InvertColors),
            "invert_colors_off" => Ok(Self::InvertColorsOff),
            "invert_colors_on" => Ok(Self::InvertColorsOn),
            "ios_share" => Ok(Self::IosShare),
            "iron" => Ok(Self::Iron),
            "iso" => Ok(Self::Iso),
            "kayaking" => Ok(Self::Kayaking),
            "keyboard" => Ok(Self::Keyboard),
            "keyboard_alt" => Ok(Self::KeyboardAlt),
            "keyboard_arrow_down" => Ok(Self::KeyboardArrowDown),
            "keyboard_arrow_left" => Ok(Self::KeyboardArrowLeft),
            "keyboard_arrow_right" => Ok(Self::KeyboardArrowRight),
            "keyboard_arrow_up" => Ok(Self::KeyboardArrowUp),
            "keyboard_backspace" => Ok(Self::KeyboardBackspace),
            "keyboard_capslock" => Ok(Self::KeyboardCapslock),
            "keyboard_control" => Ok(Self::KeyboardControl),
            "keyboard_hide" => Ok(Self::KeyboardHide),
            "keyboard_return" => Ok(Self::KeyboardReturn),
            "keyboard_tab" => Ok(Self::KeyboardTab),
            "keyboard_voice" => Ok(Self::KeyboardVoice),
            "king_bed" => Ok(Self::KingBed),
            "kitchen" => Ok(Self::Kitchen),
            "kitesurfing" => Ok(Self::Kitesurfing),
            "label" => Ok(Self::Label),
            "label_important" => Ok(Self::LabelImportant),
            "label_important_outline" => Ok(Self::LabelImportantOutline),
            "label_off" => Ok(Self::LabelOff),
            "label_outline" => Ok(Self::LabelOutline),
            "landscape" => Ok(Self::Landscape),
            "language" => Ok(Self::Language),
            "laptop" => Ok(Self::Laptop),
            "laptop_chromebook" => Ok(Self::LaptopChromebook),
            "laptop_mac" => Ok(Self::LaptopMac),
            "laptop_windows" => Ok(Self::LaptopWindows),
            "last_page" => Ok(Self::LastPage),
            "launch" => Ok(Self::Launch),
            "layers" => Ok(Self::Layers),
            "layers_clear" => Ok(Self::LayersClear),
            "leaderboard" => Ok(Self::Leaderboard),
            "leak_add" => Ok(Self::LeakAdd),
            "leak_remove" => Ok(Self::LeakRemove),
            "leave_bags_at_home" => Ok(Self::LeaveBagsAtHome),
            "legend_toggle" => Ok(Self::LegendToggle),
            "lens" => Ok(Self::Lens),
            "lens_blur" => Ok(Self::LensBlur),
            "library_add" => Ok(Self::LibraryAdd),
            "library_add_check" => Ok(Self::LibraryAddCheck),
            "library_books" => Ok(Self::LibraryBooks),
            "library_music" => Ok(Self::LibraryMusic),
            "light" => Ok(Self::Light),
            "light_mode" => Ok(Self::LightMode),
            "lightbulb" => Ok(Self::Lightbulb),
            "lightbulb_outline" => Ok(Self::LightbulbOutline),
            "line_style" => Ok(Self::LineStyle),
            "line_weight" => Ok(Self::LineWeight),
            "linear_scale" => Ok(Self::LinearScale),
            "link" => Ok(Self::Link),
            "link_off" => Ok(Self::LinkOff),
            "linked_camera" => Ok(Self::LinkedCamera),
            "liquor" => Ok(Self::Liquor),
            "list" => Ok(Self::List),
            "list_alt" => Ok(Self::ListAlt),
            "live_help" => Ok(Self::LiveHelp),
            "live_tv" => Ok(Self::LiveTv),
            "living" => Ok(Self::Living),
            "local_activity" => Ok(Self::LocalActivity),
            "local_airport" => Ok(Self::LocalAirport),
            "local_atm" => Ok(Self::LocalAtm),
            "local_attraction" => Ok(Self::LocalAttraction),
            "local_bar" => Ok(Self::LocalBar),
            "local_cafe" => Ok(Self::LocalCafe),
            "local_car_wash" => Ok(Self::LocalCarWash),
            "local_convenience_store" => Ok(Self::LocalConvenienceStore),
            "local_dining" => Ok(Self::LocalDining),
            "local_drink" => Ok(Self::LocalDrink),
            "local_fire_department" => Ok(Self::LocalFireDepartment),
            "local_florist" => Ok(Self::LocalFlorist),
            "local_gas_station" => Ok(Self::LocalGasStation),
            "local_grocery_store" => Ok(Self::LocalGroceryStore),
            "local_hospital" => Ok(Self::LocalHospital),
            "local_hotel" => Ok(Self::LocalHotel),
            "local_laundry_service" => Ok(Self::LocalLaundryService),
            "local_library" => Ok(Self::LocalLibrary),
            "local_mall" => Ok(Self::LocalMall),
            "local_movies" => Ok(Self::LocalMovies),
            "local_offer" => Ok(Self::LocalOffer),
            "local_parking" => Ok(Self::LocalParking),
            "local_pharmacy" => Ok(Self::LocalPharmacy),
            "local_phone" => Ok(Self::LocalPhone),
            "local_pizza" => Ok(Self::LocalPizza),
            "local_play" => Ok(Self::LocalPlay),
            "local_police" => Ok(Self::LocalPolice),
            "local_post_office" => Ok(Self::LocalPostOffice),
            "local_print_shop" => Ok(Self::LocalPrintShop),
            "local_printshop" => Ok(Self::LocalPrintshop),
            "local_restaurant" => Ok(Self::LocalRestaurant),
            "local_see" => Ok(Self::LocalSee),
            "local_shipping" => Ok(Self::LocalShipping),
            "local_taxi" => Ok(Self::LocalTaxi),
            "location_city" => Ok(Self::LocationCity),
            "location_disabled" => Ok(Self::LocationDisabled),
            "location_history" => Ok(Self::LocationHistory),
            "location_off" => Ok(Self::LocationOff),
            "location_on" => Ok(Self::LocationOn),
            "location_pin" => Ok(Self::LocationPin),
            "location_searching" => Ok(Self::LocationSearching),
            "lock" => Ok(Self::Lock),
            "lock_clock" => Ok(Self::LockClock),
            "lock_open" => Ok(Self::LockOpen),
            "lock_outline" => Ok(Self::LockOutline),
            "login" => Ok(Self::Login),
            "logout" => Ok(Self::Logout),
            "looks" => Ok(Self::Looks),
            "looks_3" => Ok(Self::Looks3),
            "looks_4" => Ok(Self::Looks4),
            "looks_5" => Ok(Self::Looks5),
            "looks_6" => Ok(Self::Looks6),
            "looks_one" => Ok(Self::LooksOne),
            "looks_two" => Ok(Self::LooksTwo),
            "loop" => Ok(Self::Loop),
            "loupe" => Ok(Self::Loupe),
            "low_priority" => Ok(Self::LowPriority),
            "loyalty" => Ok(Self::Loyalty),
            "lte_mobiledata" => Ok(Self::LteMobiledata),
            "lte_plus_mobiledata" => Ok(Self::LtePlusMobiledata),
            "luggage" => Ok(Self::Luggage),
            "lunch_dining" => Ok(Self::LunchDining),
            "mail" => Ok(Self::Mail),
            "mail_outline" => Ok(Self::MailOutline),
            "male" => Ok(Self::Male),
            "manage_accounts" => Ok(Self::ManageAccounts),
            "manage_search" => Ok(Self::ManageSearch),
            "map" => Ok(Self::Map),
            "maps_home_work" => Ok(Self::MapsHomeWork),
            "maps_ugc" => Ok(Self::MapsUgc),
            "margin" => Ok(Self::Margin),
            "mark_as_unread" => Ok(Self::MarkAsUnread),
            "mark_chat_read" => Ok(Self::MarkChatRead),
            "mark_chat_unread" => Ok(Self::MarkChatUnread),
            "mark_email_read" => Ok(Self::MarkEmailRead),
            "mark_email_unread" => Ok(Self::MarkEmailUnread),
            "markunread" => Ok(Self::Markunread),
            "markunread_mailbox" => Ok(Self::MarkunreadMailbox),
            "masks" => Ok(Self::Masks),
            "maximize" => Ok(Self::Maximize),
            "media_bluetooth_off" => Ok(Self::MediaBluetoothOff),
            "media_bluetooth_on" => Ok(Self::MediaBluetoothOn),
            "mediation" => Ok(Self::Mediation),
            "medical_services" => Ok(Self::MedicalServices),
            "medication" => Ok(Self::Medication),
            "meeting_room" => Ok(Self::MeetingRoom),
            "memory" => Ok(Self::Memory),
            "menu" => Ok(Self::Menu),
            "menu_book" => Ok(Self::MenuBook),
            "menu_open" => Ok(Self::MenuOpen),
            "merge_type" => Ok(Self::MergeType),
            "message" => Ok(Self::Message),
            "messenger" => Ok(Self::Messenger),
            "messenger_outline" => Ok(Self::MessengerOutline),
            "mic" => Ok(Self::Mic),
            "mic_external_off" => Ok(Self::MicExternalOff),
            "mic_external_on" => Ok(Self::MicExternalOn),
            "mic_none" => Ok(Self::MicNone),
            "mic_off" => Ok(Self::MicOff),
            "microwave" => Ok(Self::Microwave),
            "military_tech" => Ok(Self::MilitaryTech),
            "minimize" => Ok(Self::Minimize),
            "miscellaneous_services" => Ok(Self::MiscellaneousServices),
            "missed_video_call" => Ok(Self::MissedVideoCall),
            "mms" => Ok(Self::Mms),
            "mobile_friendly" => Ok(Self::MobileFriendly),
            "mobile_off" => Ok(Self::MobileOff),
            "mobile_screen_share" => Ok(Self::MobileScreenShare),
            "mobiledata_off" => Ok(Self::MobiledataOff),
            "mode" => Ok(Self::Mode),
            "mode_comment" => Ok(Self::ModeComment),
            "mode_edit" => Ok(Self::ModeEdit),
            "mode_edit_outline" => Ok(Self::ModeEditOutline),
            "mode_night" => Ok(Self::ModeNight),
            "mode_standby" => Ok(Self::ModeStandby),
            "model_training" => Ok(Self::ModelTraining),
            "monetization_on" => Ok(Self::MonetizationOn),
            "money" => Ok(Self::Money),
            "money_off" => Ok(Self::MoneyOff),
            "money_off_csred" => Ok(Self::MoneyOffCsred),
            "monitor" => Ok(Self::Monitor),
            "monitor_weight" => Ok(Self::MonitorWeight),
            "monochrome_photos" => Ok(Self::MonochromePhotos),
            "mood" => Ok(Self::Mood),
            "mood_bad" => Ok(Self::MoodBad),
            "moped" => Ok(Self::Moped),
            "more" => Ok(Self::More),
            "more_horiz" => Ok(Self::MoreHoriz),
            "more_time" => Ok(Self::MoreTime),
            "more_vert" => Ok(Self::MoreVert),
            "motion_photos_auto" => Ok(Self::MotionPhotosAuto),
            "motion_photos_off" => Ok(Self::MotionPhotosOff),
            "motion_photos_on" => Ok(Self::MotionPhotosOn),
            "motion_photos_pause" => Ok(Self::MotionPhotosPause),
            "motion_photos_paused" => Ok(Self::MotionPhotosPaused),
            "motorcycle" => Ok(Self::Motorcycle),
            "mouse" => Ok(Self::Mouse),
            "move_to_inbox" => Ok(Self::MoveToInbox),
            "movie" => Ok(Self::Movie),
            "movie_creation" => Ok(Self::MovieCreation),
            "movie_filter" => Ok(Self::MovieFilter),
            "moving" => Ok(Self::Moving),
            "mp" => Ok(Self::Mp),
            "multiline_chart" => Ok(Self::MultilineChart),
            "multiple_stop" => Ok(Self::MultipleStop),
            "multitrack_audio" => Ok(Self::MultitrackAudio),
            "museum" => Ok(Self::Museum),
            "music_note" => Ok(Self::MusicNote),
            "music_off" => Ok(Self::MusicOff),
            "music_video" => Ok(Self::MusicVideo),
            "my_library_add" => Ok(Self::MyLibraryAdd),
            "my_library_books" => Ok(Self::MyLibraryBooks),
            "my_library_music" => Ok(Self::MyLibraryMusic),
            "my_location" => Ok(Self::MyLocation),
            "nat" => Ok(Self::Nat),
            "nature" => Ok(Self::Nature),
            "nature_people" => Ok(Self::NaturePeople),
            "navigate_before" => Ok(Self::NavigateBefore),
            "navigate_next" => Ok(Self::NavigateNext),
            "navigation" => Ok(Self::Navigation),
            "near_me" => Ok(Self::NearMe),
            "near_me_disabled" => Ok(Self::NearMeDisabled),
            "nearby_error" => Ok(Self::NearbyError),
            "nearby_off" => Ok(Self::NearbyOff),
            "network_cell" => Ok(Self::NetworkCell),
            "network_check" => Ok(Self::NetworkCheck),
            "network_locked" => Ok(Self::NetworkLocked),
            "network_wifi" => Ok(Self::NetworkWifi),
            "new_label" => Ok(Self::NewLabel),
            "new_releases" => Ok(Self::NewReleases),
            "next_plan" => Ok(Self::NextPlan),
            "next_week" => Ok(Self::NextWeek),
            "nfc" => Ok(Self::Nfc),
            "night_shelter" => Ok(Self::NightShelter),
            "nightlife" => Ok(Self::Nightlife),
            "nightlight" => Ok(Self::Nightlight),
            "nightlight_round" => Ok(Self::NightlightRound),
            "nights_stay" => Ok(Self::NightsStay),
            "nine_k" => Ok(Self::NineK),
            "nine_k_plus" => Ok(Self::NineKPlus),
            "nine_mp" => Ok(Self::NineMp),
            "nineteen_mp" => Ok(Self::NineteenMp),
            "no_accounts" => Ok(Self::NoAccounts),
            "no_backpack" => Ok(Self::NoBackpack),
            "no_cell" => Ok(Self::NoCell),
            "no_drinks" => Ok(Self::NoDrinks),
            "no_encryption" => Ok(Self::NoEncryption),
            "no_encryption_gmailerrorred" => Ok(Self::NoEncryptionGmailerrorred),
            "no_flash" => Ok(Self::NoFlash),
            "no_food" => Ok(Self::NoFood),
            "no_luggage" => Ok(Self::NoLuggage),
            "no_meals" => Ok(Self::NoMeals),
            "no_meals_ouline" => Ok(Self::NoMealsOuline),
            "no_meeting_room" => Ok(Self::NoMeetingRoom),
            "no_photography" => Ok(Self::NoPhotography),
            "no_sim" => Ok(Self::NoSim),
            "no_stroller" => Ok(Self::NoStroller),
            "no_transfer" => Ok(Self::NoTransfer),
            "nordic_walking" => Ok(Self::NordicWalking),
            "north" => Ok(Self::North),
            "north_east" => Ok(Self::NorthEast),
            "north_west" => Ok(Self::NorthWest),
            "not_accessible" => Ok(Self::NotAccessible),
            "not_interested" => Ok(Self::NotInterested),
            "not_listed_location" => Ok(Self::NotListedLocation),
            "not_started" => Ok(Self::NotStarted),
            "note" => Ok(Self::Note),
            "note_add" => Ok(Self::NoteAdd),
            "note_alt" => Ok(Self::NoteAlt),
            "notes" => Ok(Self::Notes),
            "notification_add" => Ok(Self::NotificationAdd),
            "notification_important" => Ok(Self::NotificationImportant),
            "notifications" => Ok(Self::Notifications),
            "notifications_active" => Ok(Self::NotificationsActive),
            "notifications_none" => Ok(Self::NotificationsNone),
            "notifications_off" => Ok(Self::NotificationsOff),
            "notifications_on" => Ok(Self::NotificationsOn),
            "notifications_paused" => Ok(Self::NotificationsPaused),
            "now_wallpaper" => Ok(Self::NowWallpaper),
            "now_widgets" => Ok(Self::NowWidgets),
            "offline_bolt" => Ok(Self::OfflineBolt),
            "offline_pin" => Ok(Self::OfflinePin),
            "offline_share" => Ok(Self::OfflineShare),
            "ondemand_video" => Ok(Self::OndemandVideo),
            "one_k" => Ok(Self::OneK),
            "one_k_plus" => Ok(Self::OneKPlus),
            "one_x_mobiledata" => Ok(Self::OneXMobiledata),
            "online_prediction" => Ok(Self::OnlinePrediction),
            "opacity" => Ok(Self::Opacity),
            "open_in_browser" => Ok(Self::OpenInBrowser),
            "open_in_full" => Ok(Self::OpenInFull),
            "open_in_new" => Ok(Self::OpenInNew),
            "open_in_new_off" => Ok(Self::OpenInNewOff),
            "open_with" => Ok(Self::OpenWith),
            "other_houses" => Ok(Self::OtherHouses),
            "outbond" => Ok(Self::Outbond),
            "outbound" => Ok(Self::Outbound),
            "outbox" => Ok(Self::Outbox),
            "outdoor_grill" => Ok(Self::OutdoorGrill),
            "outgoing_mail" => Ok(Self::OutgoingMail),
            "outlet" => Ok(Self::Outlet),
            "outlined_flag" => Ok(Self::OutlinedFlag),
            "padding" => Ok(Self::Padding),
            "pages" => Ok(Self::Pages),
            "pageview" => Ok(Self::Pageview),
            "paid" => Ok(Self::Paid),
            "palette" => Ok(Self::Palette),
            "pan_tool" => Ok(Self::PanTool),
            "panorama" => Ok(Self::Panorama),
            "panorama_fish_eye" => Ok(Self::PanoramaFishEye),
            "panorama_fisheye" => Ok(Self::PanoramaFisheye),
            "panorama_horizontal" => Ok(Self::PanoramaHorizontal),
            "panorama_horizontal_select" => Ok(Self::PanoramaHorizontalSelect),
            "panorama_photosphere" => Ok(Self::PanoramaPhotosphere),
            "panorama_photosphere_select" => Ok(Self::PanoramaPhotosphereSelect),
            "panorama_vertical" => Ok(Self::PanoramaVertical),
            "panorama_vertical_select" => Ok(Self::PanoramaVerticalSelect),
            "panorama_wide_angle" => Ok(Self::PanoramaWideAngle),
            "panorama_wide_angle_select" => Ok(Self::PanoramaWideAngleSelect),
            "paragliding" => Ok(Self::Paragliding),
            "park" => Ok(Self::Park),
            "party_mode" => Ok(Self::PartyMode),
            "password" => Ok(Self::Password),
            "paste" => Ok(Self::Paste),
            "pattern" => Ok(Self::Pattern),
            "pause" => Ok(Self::Pause),
            "pause_circle" => Ok(Self::PauseCircle),
            "pause_circle_filled" => Ok(Self::PauseCircleFilled),
            "pause_circle_outline" => Ok(Self::PauseCircleOutline),
            "pause_presentation" => Ok(Self::PausePresentation),
            "payment" => Ok(Self::Payment),
            "payments" => Ok(Self::Payments),
            "pedal_bike" => Ok(Self::PedalBike),
            "pending" => Ok(Self::Pending),
            "pending_actions" => Ok(Self::PendingActions),
            "people" => Ok(Self::People),
            "people_alt" => Ok(Self::PeopleAlt),
            "people_outline" => Ok(Self::PeopleOutline),
            "perm_camera_mic" => Ok(Self::PermCameraMic),
            "perm_contact_cal" => Ok(Self::PermContactCal),
            "perm_contact_calendar" => Ok(Self::PermContactCalendar),
            "perm_data_setting" => Ok(Self::PermDataSetting),
            "perm_device_info" => Ok(Self::PermDeviceInfo),
            "perm_device_information" => Ok(Self::PermDeviceInformation),
            "perm_identity" => Ok(Self::PermIdentity),
            "perm_media" => Ok(Self::PermMedia),
            "perm_phone_msg" => Ok(Self::PermPhoneMsg),
            "perm_scan_wifi" => Ok(Self::PermScanWifi),
            "person" => Ok(Self::Person),
            "person_add" => Ok(Self::PersonAdd),
            "person_add_alt" => Ok(Self::PersonAddAlt),
            "person_add_alt_1" => Ok(Self::PersonAddAlt1),
            "person_add_disabled" => Ok(Self::PersonAddDisabled),
            "person_off" => Ok(Self::PersonOff),
            "person_outline" => Ok(Self::PersonOutline),
            "person_pin" => Ok(Self::PersonPin),
            "person_pin_circle" => Ok(Self::PersonPinCircle),
            "person_remove" => Ok(Self::PersonRemove),
            "person_remove_alt_1" => Ok(Self::PersonRemoveAlt1),
            "person_search" => Ok(Self::PersonSearch),
            "personal_injury" => Ok(Self::PersonalInjury),
            "personal_video" => Ok(Self::PersonalVideo),
            "pest_control" => Ok(Self::PestControl),
            "pest_control_rodent" => Ok(Self::PestControlRodent),
            "pets" => Ok(Self::Pets),
            "phone" => Ok(Self::Phone),
            "phone_android" => Ok(Self::PhoneAndroid),
            "phone_bluetooth_speaker" => Ok(Self::PhoneBluetoothSpeaker),
            "phone_callback" => Ok(Self::PhoneCallback),
            "phone_disabled" => Ok(Self::PhoneDisabled),
            "phone_enabled" => Ok(Self::PhoneEnabled),
            "phone_forwarded" => Ok(Self::PhoneForwarded),
            "phone_in_talk" => Ok(Self::PhoneInTalk),
            "phone_iphone" => Ok(Self::PhoneIphone),
            "phone_locked" => Ok(Self::PhoneLocked),
            "phone_missed" => Ok(Self::PhoneMissed),
            "phone_paused" => Ok(Self::PhonePaused),
            "phonelink" => Ok(Self::Phonelink),
            "phonelink_erase" => Ok(Self::PhonelinkErase),
            "phonelink_lock" => Ok(Self::PhonelinkLock),
            "phonelink_off" => Ok(Self::PhonelinkOff),
            "phonelink_ring" => Ok(Self::PhonelinkRing),
            "phonelink_setup" => Ok(Self::PhonelinkSetup),
            "photo" => Ok(Self::Photo),
            "photo_album" => Ok(Self::PhotoAlbum),
            "photo_camera" => Ok(Self::PhotoCamera),
            "photo_camera_back" => Ok(Self::PhotoCameraBack),
            "photo_camera_front" => Ok(Self::PhotoCameraFront),
            "photo_filter" => Ok(Self::PhotoFilter),
            "photo_library" => Ok(Self::PhotoLibrary),
            "photo_size_select_actual" => Ok(Self::PhotoSizeSelectActual),
            "photo_size_select_large" => Ok(Self::PhotoSizeSelectLarge),
            "photo_size_select_small" => Ok(Self::PhotoSizeSelectSmall),
            "piano" => Ok(Self::Piano),
            "piano_off" => Ok(Self::PianoOff),
            "picture_as_pdf" => Ok(Self::PictureAsPdf),
            "picture_in_picture" => Ok(Self::PictureInPicture),
            "picture_in_picture_alt" => Ok(Self::PictureInPictureAlt),
            "pie_chart" => Ok(Self::PieChart),
            "pie_chart_outline" => Ok(Self::PieChartOutline),
            "pin" => Ok(Self::Pin),
            "pin_drop" => Ok(Self::PinDrop),
            "pivot_table_chart" => Ok(Self::PivotTableChart),
            "place" => Ok(Self::Place),
            "plagiarism" => Ok(Self::Plagiarism),
            "play_arrow" => Ok(Self::PlayArrow),
            "play_circle" => Ok(Self::PlayCircle),
            "play_circle_fill" => Ok(Self::PlayCircleFill),
            "play_circle_filled" => Ok(Self::PlayCircleFilled),
            "play_circle_outline" => Ok(Self::PlayCircleOutline),
            "play_disabled" => Ok(Self::PlayDisabled),
            "play_for_work" => Ok(Self::PlayForWork),
            "play_lesson" => Ok(Self::PlayLesson),
            "playlist_add" => Ok(Self::PlaylistAdd),
            "playlist_add_check" => Ok(Self::PlaylistAddCheck),
            "playlist_play" => Ok(Self::PlaylistPlay),
            "plumbing" => Ok(Self::Plumbing),
            "plus_one" => Ok(Self::PlusOne),
            "podcasts" => Ok(Self::Podcasts),
            "point_of_sale" => Ok(Self::PointOfSale),
            "policy" => Ok(Self::Policy),
            "poll" => Ok(Self::Poll),
            "polymer" => Ok(Self::Polymer),
            "pool" => Ok(Self::Pool),
            "portable_wifi_off" => Ok(Self::PortableWifiOff),
            "portrait" => Ok(Self::Portrait),
            "post_add" => Ok(Self::PostAdd),
            "power" => Ok(Self::Power),
            "power_input" => Ok(Self::PowerInput),
            "power_off" => Ok(Self::PowerOff),
            "power_settings_new" => Ok(Self::PowerSettingsNew),
            "precision_manufacturing" => Ok(Self::PrecisionManufacturing),
            "pregnant_woman" => Ok(Self::PregnantWoman),
            "present_to_all" => Ok(Self::PresentToAll),
            "preview" => Ok(Self::Preview),
            "price_change" => Ok(Self::PriceChange),
            "price_check" => Ok(Self::PriceCheck),
            "print" => Ok(Self::Print),
            "print_disabled" => Ok(Self::PrintDisabled),
            "priority_high" => Ok(Self::PriorityHigh),
            "privacy_tip" => Ok(Self::PrivacyTip),
            "production_quantity_limits" => Ok(Self::ProductionQuantityLimits),
            "psychology" => Ok(Self::Psychology),
            "public" => Ok(Self::Public),
            "public_off" => Ok(Self::PublicOff),
            "publish" => Ok(Self::Publish),
            "published_with_changes" => Ok(Self::PublishedWithChanges),
            "push_pin" => Ok(Self::PushPin),
            "qr_code" => Ok(Self::QrCode),
            "qr_code_2" => Ok(Self::QrCode2),
            "qr_code_scanner" => Ok(Self::QrCodeScanner),
            "query_builder" => Ok(Self::QueryBuilder),
            "query_stats" => Ok(Self::QueryStats),
            "question_answer" => Ok(Self::QuestionAnswer),
            "queue" => Ok(Self::Queue),
            "queue_music" => Ok(Self::QueueMusic),
            "queue_play_next" => Ok(Self::QueuePlayNext),
            "quick_contacts_dialer" => Ok(Self::QuickContactsDialer),
            "quick_contacts_mail" => Ok(Self::QuickContactsMail),
            "quickreply" => Ok(Self::Quickreply),
            "quiz" => Ok(Self::Quiz),
            "r_mobiledata" => Ok(Self::RMobiledata),
            "radar" => Ok(Self::Radar),
            "radio" => Ok(Self::Radio),
            "radio_button_checked" => Ok(Self::RadioButtonChecked),
            "radio_button_off" => Ok(Self::RadioButtonOff),
            "radio_button_on" => Ok(Self::RadioButtonOn),
            "radio_button_unchecked" => Ok(Self::RadioButtonUnchecked),
            "railway_alert" => Ok(Self::RailwayAlert),
            "ramen_dining" => Ok(Self::RamenDining),
            "rate_review" => Ok(Self::RateReview),
            "raw_off" => Ok(Self::RawOff),
            "raw_on" => Ok(Self::RawOn),
            "read_more" => Ok(Self::ReadMore),
            "real_estate_agent" => Ok(Self::RealEstateAgent),
            "receipt" => Ok(Self::Receipt),
            "receipt_long" => Ok(Self::ReceiptLong),
            "recent_actors" => Ok(Self::RecentActors),
            "recommend" => Ok(Self::Recommend),
            "record_voice_over" => Ok(Self::RecordVoiceOver),
            "redeem" => Ok(Self::Redeem),
            "redo" => Ok(Self::Redo),
            "reduce_capacity" => Ok(Self::ReduceCapacity),
            "refresh" => Ok(Self::Refresh),
            "remember_me" => Ok(Self::RememberMe),
            "remove" => Ok(Self::Remove),
            "remove_circle" => Ok(Self::RemoveCircle),
            "remove_circle_outline" => Ok(Self::RemoveCircleOutline),
            "remove_done" => Ok(Self::RemoveDone),
            "remove_from_queue" => Ok(Self::RemoveFromQueue),
            "remove_moderator" => Ok(Self::RemoveModerator),
            "remove_red_eye" => Ok(Self::RemoveRedEye),
            "remove_shopping_cart" => Ok(Self::RemoveShoppingCart),
            "reorder" => Ok(Self::Reorder),
            "repeat" => Ok(Self::Repeat),
            "repeat_on" => Ok(Self::RepeatOn),
            "repeat_one" => Ok(Self::RepeatOne),
            "repeat_one_on" => Ok(Self::RepeatOneOn),
            "replay" => Ok(Self::Replay),
            "replay_10" => Ok(Self::Replay10),
            "replay_30" => Ok(Self::Replay30),
            "replay_5" => Ok(Self::Replay5),
            "replay_circle_filled" => Ok(Self::ReplayCircleFilled),
            "reply" => Ok(Self::Reply),
            "reply_all" => Ok(Self::ReplyAll),
            "report" => Ok(Self::Report),
            "report_gmailerrorred" => Ok(Self::ReportGmailerrorred),
            "report_off" => Ok(Self::ReportOff),
            "report_problem" => Ok(Self::ReportProblem),
            "request_page" => Ok(Self::RequestPage),
            "request_quote" => Ok(Self::RequestQuote),
            "reset_tv" => Ok(Self::ResetTv),
            "restart_alt" => Ok(Self::RestartAlt),
            "restaurant" => Ok(Self::Restaurant),
            "restaurant_menu" => Ok(Self::RestaurantMenu),
            "restore" => Ok(Self::Restore),
            "restore_from_trash" => Ok(Self::RestoreFromTrash),
            "restore_page" => Ok(Self::RestorePage),
            "reviews" => Ok(Self::Reviews),
            "rice_bowl" => Ok(Self::RiceBowl),
            "ring_volume" => Ok(Self::RingVolume),
            "roofing" => Ok(Self::Roofing),
            "room" => Ok(Self::Room),
            "room_preferences" => Ok(Self::RoomPreferences),
            "room_service" => Ok(Self::RoomService),
            "rotate_90_degrees_ccw" => Ok(Self::Rotate90DegreesCcw),
            "rotate_left" => Ok(Self::RotateLeft),
            "rotate_right" => Ok(Self::RotateRight),
            "rounded_corner" => Ok(Self::RoundedCorner),
            "router" => Ok(Self::Router),
            "rowing" => Ok(Self::Rowing),
            "rss_feed" => Ok(Self::RssFeed),
            "rsvp" => Ok(Self::Rsvp),
            "rtt" => Ok(Self::Rtt),
            "rule" => Ok(Self::Rule),
            "rule_folder" => Ok(Self::RuleFolder),
            "run_circle" => Ok(Self::RunCircle),
            "running_with_errors" => Ok(Self::RunningWithErrors),
            "rv_hookup" => Ok(Self::RvHookup),
            "safety_divider" => Ok(Self::SafetyDivider),
            "sailing" => Ok(Self::Sailing),
            "sanitizer" => Ok(Self::Sanitizer),
            "satellite" => Ok(Self::Satellite),
            "save" => Ok(Self::Save),
            "save_alt" => Ok(Self::SaveAlt),
            "saved_search" => Ok(Self::SavedSearch),
            "savings" => Ok(Self::Savings),
            "scanner" => Ok(Self::Scanner),
            "scatter_plot" => Ok(Self::ScatterPlot),
            "schedule" => Ok(Self::Schedule),
            "schedule_send" => Ok(Self::ScheduleSend),
            "schema" => Ok(Self::Schema),
            "school" => Ok(Self::School),
            "science" => Ok(Self::Science),
            "score" => Ok(Self::Score),
            "screen_lock_landscape" => Ok(Self::ScreenLockLandscape),
            "screen_lock_portrait" => Ok(Self::ScreenLockPortrait),
            "screen_lock_rotation" => Ok(Self::ScreenLockRotation),
            "screen_rotation" => Ok(Self::ScreenRotation),
            "screen_search_desktop" => Ok(Self::ScreenSearchDesktop),
            "screen_share" => Ok(Self::ScreenShare),
            "screenshot" => Ok(Self::Screenshot),
            "sd" => Ok(Self::Sd),
            "sd_card" => Ok(Self::SdCard),
            "sd_card_alert" => Ok(Self::SdCardAlert),
            "sd_storage" => Ok(Self::SdStorage),
            "search" => Ok(Self::Search),
            "search_off" => Ok(Self::SearchOff),
            "security" => Ok(Self::Security),
            "security_update" => Ok(Self::SecurityUpdate),
            "security_update_good" => Ok(Self::SecurityUpdateGood),
            "security_update_warning" => Ok(Self::SecurityUpdateWarning),
            "segment" => Ok(Self::Segment),
            "select_all" => Ok(Self::SelectAll),
            "self_improvement" => Ok(Self::SelfImprovement),
            "sell" => Ok(Self::Sell),
            "send" => Ok(Self::Send),
            "send_and_archive" => Ok(Self::SendAndArchive),
            "send_to_mobile" => Ok(Self::SendToMobile),
            "sensor_door" => Ok(Self::SensorDoor),
            "sensor_window" => Ok(Self::SensorWindow),
            "sensors" => Ok(Self::Sensors),
            "sensors_off" => Ok(Self::SensorsOff),
            "sentiment_dissatisfied" => Ok(Self::SentimentDissatisfied),
            "sentiment_neutral" => Ok(Self::SentimentNeutral),
            "sentiment_satisfied" => Ok(Self::SentimentSatisfied),
            "sentiment_satisfied_alt" => Ok(Self::SentimentSatisfiedAlt),
            "sentiment_very_dissatisfied" => Ok(Self::SentimentVeryDissatisfied),
            "sentiment_very_satisfied" => Ok(Self::SentimentVerySatisfied),
            "set_meal" => Ok(Self::SetMeal),
            "settings" => Ok(Self::Settings),
            "settings_accessibility" => Ok(Self::SettingsAccessibility),
            "settings_applications" => Ok(Self::SettingsApplications),
            "settings_backup_restore" => Ok(Self::SettingsBackupRestore),
            "settings_bluetooth" => Ok(Self::SettingsBluetooth),
            "settings_brightness" => Ok(Self::SettingsBrightness),
            "settings_cell" => Ok(Self::SettingsCell),
            "settings_display" => Ok(Self::SettingsDisplay),
            "settings_ethernet" => Ok(Self::SettingsEthernet),
            "settings_input_antenna" => Ok(Self::SettingsInputAntenna),
            "settings_input_component" => Ok(Self::SettingsInputComponent),
            "settings_input_composite" => Ok(Self::SettingsInputComposite),
            "settings_input_hdmi" => Ok(Self::SettingsInputHdmi),
            "settings_input_svideo" => Ok(Self::SettingsInputSvideo),
            "settings_overscan" => Ok(Self::SettingsOverscan),
            "settings_phone" => Ok(Self::SettingsPhone),
            "settings_power" => Ok(Self::SettingsPower),
            "settings_remote" => Ok(Self::SettingsRemote),
            "settings_suggest" => Ok(Self::SettingsSuggest),
            "settings_system_daydream" => Ok(Self::SettingsSystemDaydream),
            "settings_voice" => Ok(Self::SettingsVoice),
            "seven_k" => Ok(Self::SevenK),
            "seven_k_plus" => Ok(Self::SevenKPlus),
            "seven_mp" => Ok(Self::SevenMp),
            "seventeen_mp" => Ok(Self::SeventeenMp),
            "share" => Ok(Self::Share),
            "share_arrival_time" => Ok(Self::ShareArrivalTime),
            "share_location" => Ok(Self::ShareLocation),
            "shield" => Ok(Self::Shield),
            "shop" => Ok(Self::Shop),
            "shop_2" => Ok(Self::Shop2),
            "shop_two" => Ok(Self::ShopTwo),
            "shopping_bag" => Ok(Self::ShoppingBag),
            "shopping_basket" => Ok(Self::ShoppingBasket),
            "shopping_cart" => Ok(Self::ShoppingCart),
            "short_text" => Ok(Self::ShortText),
            "shortcut" => Ok(Self::Shortcut),
            "show_chart" => Ok(Self::ShowChart),
            "shower" => Ok(Self::Shower),
            "shuffle" => Ok(Self::Shuffle),
            "shuffle_on" => Ok(Self::ShuffleOn),
            "shutter_speed" => Ok(Self::ShutterSpeed),
            "sick" => Ok(Self::Sick),
            "signal_cellular_0_bar" => Ok(Self::SignalCellular0Bar),
            "signal_cellular_4_bar" => Ok(Self::SignalCellular4Bar),
            "signal_cellular_alt" => Ok(Self::SignalCellularAlt),
            "signal_cellular_connected_no_internet_0_bar" => {
                Ok(Self::SignalCellularConnectedNoInternet0Bar)
            }
            "signal_cellular_connected_no_internet_4_bar" => {
                Ok(Self::SignalCellularConnectedNoInternet4Bar)
            }
            "signal_cellular_no_sim" => Ok(Self::SignalCellularNoSim),
            "signal_cellular_nodata" => Ok(Self::SignalCellularNodata),
            "signal_cellular_null" => Ok(Self::SignalCellularNull),
            "signal_cellular_off" => Ok(Self::SignalCellularOff),
            "signal_wifi_0_bar" => Ok(Self::SignalWifi0Bar),
            "signal_wifi_4_bar" => Ok(Self::SignalWifi4Bar),
            "signal_wifi_4_bar_lock" => Ok(Self::SignalWifi4BarLock),
            "signal_wifi_bad" => Ok(Self::SignalWifiBad),
            "signal_wifi_connected_no_internet_4" => {
                Ok(Self::SignalWifiConnectedNoInternet4)
            }
            "signal_wifi_off" => Ok(Self::SignalWifiOff),
            "signal_wifi_statusbar_4_bar" => Ok(Self::SignalWifiStatusbar4Bar),
            "signal_wifi_statusbar_connected_no_internet_4" => {
                Ok(Self::SignalWifiStatusbarConnectedNoInternet4)
            }
            "signal_wifi_statusbar_null" => Ok(Self::SignalWifiStatusbarNull),
            "sim_card" => Ok(Self::SimCard),
            "sim_card_alert" => Ok(Self::SimCardAlert),
            "sim_card_download" => Ok(Self::SimCardDownload),
            "single_bed" => Ok(Self::SingleBed),
            "sip" => Ok(Self::Sip),
            "six_ft_apart" => Ok(Self::SixFtApart),
            "six_k" => Ok(Self::SixK),
            "six_k_plus" => Ok(Self::SixKPlus),
            "six_mp" => Ok(Self::SixMp),
            "sixteen_mp" => Ok(Self::SixteenMp),
            "sixty_fps" => Ok(Self::SixtyFps),
            "sixty_fps_select" => Ok(Self::SixtyFpsSelect),
            "skateboarding" => Ok(Self::Skateboarding),
            "skip_next" => Ok(Self::SkipNext),
            "skip_previous" => Ok(Self::SkipPrevious),
            "sledding" => Ok(Self::Sledding),
            "slideshow" => Ok(Self::Slideshow),
            "slow_motion_video" => Ok(Self::SlowMotionVideo),
            "smart_button" => Ok(Self::SmartButton),
            "smart_display" => Ok(Self::SmartDisplay),
            "smart_screen" => Ok(Self::SmartScreen),
            "smart_toy" => Ok(Self::SmartToy),
            "smartphone" => Ok(Self::Smartphone),
            "smoke_free" => Ok(Self::SmokeFree),
            "smoking_rooms" => Ok(Self::SmokingRooms),
            "sms" => Ok(Self::Sms),
            "sms_failed" => Ok(Self::SmsFailed),
            "snippet_folder" => Ok(Self::SnippetFolder),
            "snooze" => Ok(Self::Snooze),
            "snowboarding" => Ok(Self::Snowboarding),
            "snowmobile" => Ok(Self::Snowmobile),
            "snowshoeing" => Ok(Self::Snowshoeing),
            "soap" => Ok(Self::Soap),
            "social_distance" => Ok(Self::SocialDistance),
            "sort" => Ok(Self::Sort),
            "sort_by_alpha" => Ok(Self::SortByAlpha),
            "source" => Ok(Self::Source),
            "south" => Ok(Self::South),
            "south_east" => Ok(Self::SouthEast),
            "south_west" => Ok(Self::SouthWest),
            "spa" => Ok(Self::Spa),
            "space_bar" => Ok(Self::SpaceBar),
            "space_dashboard" => Ok(Self::SpaceDashboard),
            "speaker" => Ok(Self::Speaker),
            "speaker_group" => Ok(Self::SpeakerGroup),
            "speaker_notes" => Ok(Self::SpeakerNotes),
            "speaker_notes_off" => Ok(Self::SpeakerNotesOff),
            "speaker_phone" => Ok(Self::SpeakerPhone),
            "speed" => Ok(Self::Speed),
            "spellcheck" => Ok(Self::Spellcheck),
            "splitscreen" => Ok(Self::Splitscreen),
            "sports" => Ok(Self::Sports),
            "sports_bar" => Ok(Self::SportsBar),
            "sports_baseball" => Ok(Self::SportsBaseball),
            "sports_basketball" => Ok(Self::SportsBasketball),
            "sports_cricket" => Ok(Self::SportsCricket),
            "sports_esports" => Ok(Self::SportsEsports),
            "sports_football" => Ok(Self::SportsFootball),
            "sports_golf" => Ok(Self::SportsGolf),
            "sports_handball" => Ok(Self::SportsHandball),
            "sports_hockey" => Ok(Self::SportsHockey),
            "sports_kabaddi" => Ok(Self::SportsKabaddi),
            "sports_mma" => Ok(Self::SportsMma),
            "sports_motorsports" => Ok(Self::SportsMotorsports),
            "sports_rugby" => Ok(Self::SportsRugby),
            "sports_score" => Ok(Self::SportsScore),
            "sports_soccer" => Ok(Self::SportsSoccer),
            "sports_tennis" => Ok(Self::SportsTennis),
            "sports_volleyball" => Ok(Self::SportsVolleyball),
            "square_foot" => Ok(Self::SquareFoot),
            "stacked_bar_chart" => Ok(Self::StackedBarChart),
            "stacked_line_chart" => Ok(Self::StackedLineChart),
            "stairs" => Ok(Self::Stairs),
            "star" => Ok(Self::Star),
            "star_border" => Ok(Self::StarBorder),
            "star_border_purple500" => Ok(Self::StarBorderPurple500),
            "star_half" => Ok(Self::StarHalf),
            "star_outline" => Ok(Self::StarOutline),
            "star_purple500" => Ok(Self::StarPurple500),
            "star_rate" => Ok(Self::StarRate),
            "stars" => Ok(Self::Stars),
            "stay_current_landscape" => Ok(Self::StayCurrentLandscape),
            "stay_current_portrait" => Ok(Self::StayCurrentPortrait),
            "stay_primary_landscape" => Ok(Self::StayPrimaryLandscape),
            "stay_primary_portrait" => Ok(Self::StayPrimaryPortrait),
            "sticky_note_2" => Ok(Self::StickyNote2),
            "stop" => Ok(Self::Stop),
            "stop_circle" => Ok(Self::StopCircle),
            "stop_screen_share" => Ok(Self::StopScreenShare),
            "storage" => Ok(Self::Storage),
            "store" => Ok(Self::Store),
            "store_mall_directory" => Ok(Self::StoreMallDirectory),
            "storefront" => Ok(Self::Storefront),
            "storm" => Ok(Self::Storm),
            "straighten" => Ok(Self::Straighten),
            "stream" => Ok(Self::Stream),
            "streetview" => Ok(Self::Streetview),
            "strikethrough_s" => Ok(Self::StrikethroughS),
            "stroller" => Ok(Self::Stroller),
            "style" => Ok(Self::Style),
            "subdirectory_arrow_left" => Ok(Self::SubdirectoryArrowLeft),
            "subdirectory_arrow_right" => Ok(Self::SubdirectoryArrowRight),
            "subject" => Ok(Self::Subject),
            "subscript" => Ok(Self::Subscript),
            "subscriptions" => Ok(Self::Subscriptions),
            "subtitles" => Ok(Self::Subtitles),
            "subtitles_off" => Ok(Self::SubtitlesOff),
            "subway" => Ok(Self::Subway),
            "summarize" => Ok(Self::Summarize),
            "superscript" => Ok(Self::Superscript),
            "supervised_user_circle" => Ok(Self::SupervisedUserCircle),
            "supervisor_account" => Ok(Self::SupervisorAccount),
            "support" => Ok(Self::Support),
            "support_agent" => Ok(Self::SupportAgent),
            "surfing" => Ok(Self::Surfing),
            "surround_sound" => Ok(Self::SurroundSound),
            "swap_calls" => Ok(Self::SwapCalls),
            "swap_horiz" => Ok(Self::SwapHoriz),
            "swap_horizontal_circle" => Ok(Self::SwapHorizontalCircle),
            "swap_vert" => Ok(Self::SwapVert),
            "swap_vert_circle" => Ok(Self::SwapVertCircle),
            "swap_vertical_circle" => Ok(Self::SwapVerticalCircle),
            "swipe" => Ok(Self::Swipe),
            "switch_account" => Ok(Self::SwitchAccount),
            "switch_camera" => Ok(Self::SwitchCamera),
            "switch_left" => Ok(Self::SwitchLeft),
            "switch_right" => Ok(Self::SwitchRight),
            "switch_video" => Ok(Self::SwitchVideo),
            "sync" => Ok(Self::Sync),
            "sync_alt" => Ok(Self::SyncAlt),
            "sync_disabled" => Ok(Self::SyncDisabled),
            "sync_problem" => Ok(Self::SyncProblem),
            "system_security_update" => Ok(Self::SystemSecurityUpdate),
            "system_security_update_good" => Ok(Self::SystemSecurityUpdateGood),
            "system_security_update_warning" => Ok(Self::SystemSecurityUpdateWarning),
            "system_update" => Ok(Self::SystemUpdate),
            "system_update_alt" => Ok(Self::SystemUpdateAlt),
            "system_update_tv" => Ok(Self::SystemUpdateTv),
            "tab" => Ok(Self::Tab),
            "tab_unselected" => Ok(Self::TabUnselected),
            "table_chart" => Ok(Self::TableChart),
            "table_rows" => Ok(Self::TableRows),
            "table_view" => Ok(Self::TableView),
            "tablet" => Ok(Self::Tablet),
            "tablet_android" => Ok(Self::TabletAndroid),
            "tablet_mac" => Ok(Self::TabletMac),
            "tag" => Ok(Self::Tag),
            "tag_faces" => Ok(Self::TagFaces),
            "takeout_dining" => Ok(Self::TakeoutDining),
            "tap_and_play" => Ok(Self::TapAndPlay),
            "tapas" => Ok(Self::Tapas),
            "task" => Ok(Self::Task),
            "task_alt" => Ok(Self::TaskAlt),
            "taxi_alert" => Ok(Self::TaxiAlert),
            "ten_k" => Ok(Self::TenK),
            "ten_mp" => Ok(Self::TenMp),
            "terrain" => Ok(Self::Terrain),
            "text_fields" => Ok(Self::TextFields),
            "text_format" => Ok(Self::TextFormat),
            "text_rotate_up" => Ok(Self::TextRotateUp),
            "text_rotate_vertical" => Ok(Self::TextRotateVertical),
            "text_rotation_angledown" => Ok(Self::TextRotationAngledown),
            "text_rotation_angleup" => Ok(Self::TextRotationAngleup),
            "text_rotation_down" => Ok(Self::TextRotationDown),
            "text_rotation_none" => Ok(Self::TextRotationNone),
            "text_snippet" => Ok(Self::TextSnippet),
            "textsms" => Ok(Self::Textsms),
            "texture" => Ok(Self::Texture),
            "theater_comedy" => Ok(Self::TheaterComedy),
            "theaters" => Ok(Self::Theaters),
            "thermostat" => Ok(Self::Thermostat),
            "thermostat_auto" => Ok(Self::ThermostatAuto),
            "thirteen_mp" => Ok(Self::ThirteenMp),
            "thirty_fps" => Ok(Self::ThirtyFps),
            "thirty_fps_select" => Ok(Self::ThirtyFpsSelect),
            "three_g_mobiledata" => Ok(Self::ThreeGMobiledata),
            "three_k" => Ok(Self::ThreeK),
            "three_k_plus" => Ok(Self::ThreeKPlus),
            "three_mp" => Ok(Self::ThreeMp),
            "three_p" => Ok(Self::ThreeP),
            "threed_rotation" => Ok(Self::ThreedRotation),
            "threesixty" => Ok(Self::Threesixty),
            "thumb_down" => Ok(Self::ThumbDown),
            "thumb_down_alt" => Ok(Self::ThumbDownAlt),
            "thumb_down_off_alt" => Ok(Self::ThumbDownOffAlt),
            "thumb_up" => Ok(Self::ThumbUp),
            "thumb_up_alt" => Ok(Self::ThumbUpAlt),
            "thumb_up_off_alt" => Ok(Self::ThumbUpOffAlt),
            "thumbs_up_down" => Ok(Self::ThumbsUpDown),
            "time_to_leave" => Ok(Self::TimeToLeave),
            "timelapse" => Ok(Self::Timelapse),
            "timeline" => Ok(Self::Timeline),
            "timer" => Ok(Self::Timer),
            "timer_10" => Ok(Self::Timer10),
            "timer_10_select" => Ok(Self::Timer10Select),
            "timer_3" => Ok(Self::Timer3),
            "timer_3_select" => Ok(Self::Timer3Select),
            "timer_off" => Ok(Self::TimerOff),
            "title" => Ok(Self::Title),
            "toc" => Ok(Self::Toc),
            "today" => Ok(Self::Today),
            "toggle_off" => Ok(Self::ToggleOff),
            "toggle_on" => Ok(Self::ToggleOn),
            "toll" => Ok(Self::Toll),
            "tonality" => Ok(Self::Tonality),
            "topic" => Ok(Self::Topic),
            "touch_app" => Ok(Self::TouchApp),
            "tour" => Ok(Self::Tour),
            "toys" => Ok(Self::Toys),
            "track_changes" => Ok(Self::TrackChanges),
            "traffic" => Ok(Self::Traffic),
            "train" => Ok(Self::Train),
            "tram" => Ok(Self::Tram),
            "transfer_within_a_station" => Ok(Self::TransferWithinAStation),
            "transform" => Ok(Self::Transform),
            "transgender" => Ok(Self::Transgender),
            "transit_enterexit" => Ok(Self::TransitEnterexit),
            "translate" => Ok(Self::Translate),
            "travel_explore" => Ok(Self::TravelExplore),
            "trending_down" => Ok(Self::TrendingDown),
            "trending_flat" => Ok(Self::TrendingFlat),
            "trending_neutral" => Ok(Self::TrendingNeutral),
            "trending_up" => Ok(Self::TrendingUp),
            "trip_origin" => Ok(Self::TripOrigin),
            "try_sms_star" => Ok(Self::TrySmsStar),
            "tty" => Ok(Self::Tty),
            "tune" => Ok(Self::Tune),
            "tungsten" => Ok(Self::Tungsten),
            "turned_in" => Ok(Self::TurnedIn),
            "turned_in_not" => Ok(Self::TurnedInNot),
            "tv" => Ok(Self::Tv),
            "tv_off" => Ok(Self::TvOff),
            "twelve_mp" => Ok(Self::TwelveMp),
            "twenty_four_mp" => Ok(Self::TwentyFourMp),
            "twenty_mp" => Ok(Self::TwentyMp),
            "twenty_one_mp" => Ok(Self::TwentyOneMp),
            "twenty_three_mp" => Ok(Self::TwentyThreeMp),
            "twenty_two_mp" => Ok(Self::TwentyTwoMp),
            "two_k" => Ok(Self::TwoK),
            "two_k_plus" => Ok(Self::TwoKPlus),
            "two_mp" => Ok(Self::TwoMp),
            "two_wheeler" => Ok(Self::TwoWheeler),
            "umbrella" => Ok(Self::Umbrella),
            "unarchive" => Ok(Self::Unarchive),
            "undo" => Ok(Self::Undo),
            "unfold_less" => Ok(Self::UnfoldLess),
            "unfold_more" => Ok(Self::UnfoldMore),
            "unpublished" => Ok(Self::Unpublished),
            "unsubscribe" => Ok(Self::Unsubscribe),
            "upcoming" => Ok(Self::Upcoming),
            "update" => Ok(Self::Update),
            "update_disabled" => Ok(Self::UpdateDisabled),
            "upgrade" => Ok(Self::Upgrade),
            "upload" => Ok(Self::Upload),
            "upload_file" => Ok(Self::UploadFile),
            "usb" => Ok(Self::Usb),
            "usb_off" => Ok(Self::UsbOff),
            "verified" => Ok(Self::Verified),
            "verified_user" => Ok(Self::VerifiedUser),
            "vertical_align_bottom" => Ok(Self::VerticalAlignBottom),
            "vertical_align_center" => Ok(Self::VerticalAlignCenter),
            "vertical_align_top" => Ok(Self::VerticalAlignTop),
            "vertical_distribute" => Ok(Self::VerticalDistribute),
            "vertical_split" => Ok(Self::VerticalSplit),
            "vibration" => Ok(Self::Vibration),
            "video_call" => Ok(Self::VideoCall),
            "video_camera_back" => Ok(Self::VideoCameraBack),
            "video_camera_front" => Ok(Self::VideoCameraFront),
            "video_collection" => Ok(Self::VideoCollection),
            "video_label" => Ok(Self::VideoLabel),
            "video_library" => Ok(Self::VideoLibrary),
            "video_settings" => Ok(Self::VideoSettings),
            "video_stable" => Ok(Self::VideoStable),
            "videocam" => Ok(Self::Videocam),
            "videocam_off" => Ok(Self::VideocamOff),
            "videogame_asset" => Ok(Self::VideogameAsset),
            "videogame_asset_off" => Ok(Self::VideogameAssetOff),
            "view_agenda" => Ok(Self::ViewAgenda),
            "view_array" => Ok(Self::ViewArray),
            "view_carousel" => Ok(Self::ViewCarousel),
            "view_column" => Ok(Self::ViewColumn),
            "view_comfortable" => Ok(Self::ViewComfortable),
            "view_comfy" => Ok(Self::ViewComfy),
            "view_compact" => Ok(Self::ViewCompact),
            "view_day" => Ok(Self::ViewDay),
            "view_headline" => Ok(Self::ViewHeadline),
            "view_in_ar" => Ok(Self::ViewInAr),
            "view_list" => Ok(Self::ViewList),
            "view_module" => Ok(Self::ViewModule),
            "view_quilt" => Ok(Self::ViewQuilt),
            "view_sidebar" => Ok(Self::ViewSidebar),
            "view_stream" => Ok(Self::ViewStream),
            "view_week" => Ok(Self::ViewWeek),
            "vignette" => Ok(Self::Vignette),
            "villa" => Ok(Self::Villa),
            "visibility" => Ok(Self::Visibility),
            "visibility_off" => Ok(Self::VisibilityOff),
            "voice_chat" => Ok(Self::VoiceChat),
            "voice_over_off" => Ok(Self::VoiceOverOff),
            "voicemail" => Ok(Self::Voicemail),
            "volume_down" => Ok(Self::VolumeDown),
            "volume_mute" => Ok(Self::VolumeMute),
            "volume_off" => Ok(Self::VolumeOff),
            "volume_up" => Ok(Self::VolumeUp),
            "volunteer_activism" => Ok(Self::VolunteerActivism),
            "vpn_key" => Ok(Self::VpnKey),
            "vpn_lock" => Ok(Self::VpnLock),
            "vrpano" => Ok(Self::Vrpano),
            "wallet_giftcard" => Ok(Self::WalletGiftcard),
            "wallet_membership" => Ok(Self::WalletMembership),
            "wallet_travel" => Ok(Self::WalletTravel),
            "wallpaper" => Ok(Self::Wallpaper),
            "warning" => Ok(Self::Warning),
            "warning_amber" => Ok(Self::WarningAmber),
            "wash" => Ok(Self::Wash),
            "watch" => Ok(Self::Watch),
            "watch_later" => Ok(Self::WatchLater),
            "water" => Ok(Self::Water),
            "water_damage" => Ok(Self::WaterDamage),
            "waterfall_chart" => Ok(Self::WaterfallChart),
            "waves" => Ok(Self::Waves),
            "wb_auto" => Ok(Self::WbAuto),
            "wb_cloudy" => Ok(Self::WbCloudy),
            "wb_incandescent" => Ok(Self::WbIncandescent),
            "wb_iridescent" => Ok(Self::WbIridescent),
            "wb_shade" => Ok(Self::WbShade),
            "wb_sunny" => Ok(Self::WbSunny),
            "wb_twighlight" => Ok(Self::WbTwighlight),
            "wb_twilight" => Ok(Self::WbTwilight),
            "wc" => Ok(Self::Wc),
            "web" => Ok(Self::Web),
            "web_asset" => Ok(Self::WebAsset),
            "web_asset_off" => Ok(Self::WebAssetOff),
            "web_stories" => Ok(Self::WebStories),
            "weekend" => Ok(Self::Weekend),
            "west" => Ok(Self::West),
            "whatshot" => Ok(Self::Whatshot),
            "wheelchair_pickup" => Ok(Self::WheelchairPickup),
            "where_to_vote" => Ok(Self::WhereToVote),
            "widgets" => Ok(Self::Widgets),
            "wifi" => Ok(Self::Wifi),
            "wifi_calling" => Ok(Self::WifiCalling),
            "wifi_calling_3" => Ok(Self::WifiCalling3),
            "wifi_lock" => Ok(Self::WifiLock),
            "wifi_off" => Ok(Self::WifiOff),
            "wifi_protected_setup" => Ok(Self::WifiProtectedSetup),
            "wifi_tethering" => Ok(Self::WifiTethering),
            "wifi_tethering_off" => Ok(Self::WifiTetheringOff),
            "window" => Ok(Self::Window),
            "wine_bar" => Ok(Self::WineBar),
            "work" => Ok(Self::Work),
            "work_off" => Ok(Self::WorkOff),
            "work_outline" => Ok(Self::WorkOutline),
            "workspaces" => Ok(Self::Workspaces),
            "workspaces_filled" => Ok(Self::WorkspacesFilled),
            "workspaces_outline" => Ok(Self::WorkspacesOutline),
            "wrap_text" => Ok(Self::WrapText),
            "wrong_location" => Ok(Self::WrongLocation),
            "wysiwyg" => Ok(Self::Wysiwyg),
            "yard" => Ok(Self::Yard),
            "youtube_searched_for" => Ok(Self::YoutubeSearchedFor),
            "zoom_in" => Ok(Self::ZoomIn),
            "zoom_out" => Ok(Self::ZoomOut),
            "zoom_out_map" => Ok(Self::ZoomOutMap),
            "zoom_out_outlined" => Ok(Self::ZoomOutOutlined),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesIconName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesIconName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesIconName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///How the image should be painted on the areas that it does not cover.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesImageRepeat {
    #[serde(rename = "noRepeat")]
    NoRepeat,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "repeatX")]
    RepeatX,
    #[serde(rename = "repeatY")]
    RepeatY,
}
impl From<&StylesImageRepeat> for StylesImageRepeat {
    fn from(value: &StylesImageRepeat) -> Self {
        value.clone()
    }
}
impl ToString for StylesImageRepeat {
    fn to_string(&self) -> String {
        match *self {
            Self::NoRepeat => "noRepeat".to_string(),
            Self::Repeat => "repeat".to_string(),
            Self::RepeatX => "repeatX".to_string(),
            Self::RepeatY => "repeatY".to_string(),
        }
    }
}
impl std::str::FromStr for StylesImageRepeat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "noRepeat" => Ok(Self::NoRepeat),
            "repeat" => Ok(Self::Repeat),
            "repeatX" => Ok(Self::RepeatX),
            "repeatY" => Ok(Self::RepeatY),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesImageRepeat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesImageRepeat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesImageRepeat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type InputBorder
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesInputBorder {
    #[serde(rename = "borderRadius", default, skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<StylesBorderRadius>,
    #[serde(rename = "borderSide")]
    pub border_side: StylesBorderSide,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<StylesInputBorderType>,
}
impl From<&StylesInputBorder> for StylesInputBorder {
    fn from(value: &StylesInputBorder) -> Self {
        value.clone()
    }
}
impl StylesInputBorder {
    pub fn builder() -> builder::StylesInputBorder {
        builder::StylesInputBorder::default()
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesInputBorderType {
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "outline")]
    Outline,
}
impl From<&StylesInputBorderType> for StylesInputBorderType {
    fn from(value: &StylesInputBorderType) -> Self {
        value.clone()
    }
}
impl ToString for StylesInputBorderType {
    fn to_string(&self) -> String {
        match *self {
            Self::Underline => "underline".to_string(),
            Self::Outline => "outline".to_string(),
        }
    }
}
impl std::str::FromStr for StylesInputBorderType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "underline" => Ok(Self::Underline),
            "outline" => Ok(Self::Outline),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesInputBorderType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesInputBorderType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesInputBorderType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type InputDecoration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesInputDecoration {
    ///Whether to align the label with the hint or not. Defaults to false.
    #[serde(
        rename = "alignLabelWithHint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub align_label_with_hint: Option<bool>,
    ///The border to display around the input. Will render a border by default when border is null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<StylesInputBorder>,
    ///The constraints to be applied to the input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<StylesBoxConstraints>,
    ///The padding to be applied to the input.
    #[serde(rename = "contentPadding", default, skip_serializing_if = "Option::is_none")]
    pub content_padding: Option<StylesPadding>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub counter: serde_json::Map<String, serde_json::Value>,
    ///The style use for the counterText.
    #[serde(rename = "counterStyle", default, skip_serializing_if = "Option::is_none")]
    pub counter_style: Option<StylesTextStyle>,
    ///The text to place below the line as a character counter.
    #[serde(rename = "counterText", default, skip_serializing_if = "Option::is_none")]
    pub counter_text: Option<String>,
    ///The border to display when the input is disabled and not showing an error.
    #[serde(rename = "disabledBorder", default, skip_serializing_if = "Option::is_none")]
    pub disabled_border: Option<StylesInputBorder>,
    ///Whether the input is enabled or disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The border to display when the input is enabled and not showing an error.
    #[serde(rename = "enabledBorder", default, skip_serializing_if = "Option::is_none")]
    pub enabled_border: Option<StylesInputBorder>,
    ///The border to display when the input has an error and does not have the focus.
    #[serde(rename = "errorBorder", default, skip_serializing_if = "Option::is_none")]
    pub error_border: Option<StylesInputBorder>,
    ///The maximum number of lines the error text can use.
    #[serde(rename = "errorMaxLines", default, skip_serializing_if = "Option::is_none")]
    pub error_max_lines: Option<i64>,
    ///The style to use for the error text.
    #[serde(rename = "errorStyle", default, skip_serializing_if = "Option::is_none")]
    pub error_style: Option<StylesTextStyle>,
    ///The error text to display when the input has an error.
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    pub error_text: Option<String>,
    ///The fill color of the input.
    #[serde(rename = "fillColor", default, skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<StylesColor>,
    ///Whether the input is filled with fillColor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,
    ///Defines how the floating label should be displayed.
    #[serde(
        rename = "floatingLabelBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_label_behavior: Option<StylesFloatingLabelBehavior>,
    ///The style to use for the floating label.
    #[serde(
        rename = "floatingLabelStyle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_label_style: Option<StylesTextStyle>,
    ///The color to use when the input is focused.
    #[serde(rename = "focusColor", default, skip_serializing_if = "Option::is_none")]
    pub focus_color: Option<StylesColor>,
    ///The border to display when the input has the focus.
    #[serde(rename = "focusedBorder", default, skip_serializing_if = "Option::is_none")]
    pub focused_border: Option<StylesInputBorder>,
    ///The border to display when the input has the focus and has an error.
    #[serde(
        rename = "focusedErrorBorder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub focused_error_border: Option<StylesInputBorder>,
    ///The maximum number of lines the helper text can use.
    #[serde(rename = "helperMaxLines", default, skip_serializing_if = "Option::is_none")]
    pub helper_max_lines: Option<i64>,
    ///The style to use for the helper text.
    #[serde(rename = "helperStyle", default, skip_serializing_if = "Option::is_none")]
    pub helper_style: Option<StylesTextStyle>,
    ///The helper text to display.
    #[serde(rename = "helperText", default, skip_serializing_if = "Option::is_none")]
    pub helper_text: Option<String>,
    ///The maximum number of lines the hint text can use.
    #[serde(rename = "hintMaxLines", default, skip_serializing_if = "Option::is_none")]
    pub hint_max_lines: Option<i64>,
    ///The style to use for the hint text.
    #[serde(rename = "hintStyle", default, skip_serializing_if = "Option::is_none")]
    pub hint_style: Option<StylesTextStyle>,
    ///The hint text to display.
    #[serde(rename = "hintText", default, skip_serializing_if = "Option::is_none")]
    pub hint_text: Option<String>,
    ///The direction of the hint text.
    #[serde(
        rename = "hintTextDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hint_text_direction: Option<StylesTextDirection>,
    ///The color to use when the input is hovered.
    #[serde(rename = "hoverColor", default, skip_serializing_if = "Option::is_none")]
    pub hover_color: Option<StylesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    ///The color for the icon
    #[serde(rename = "iconColor", default, skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<StylesColor>,
    ///Whether the decoration is the same size as the input field.
    #[serde(rename = "isCollapsed", default, skip_serializing_if = "Option::is_none")]
    pub is_collapsed: Option<bool>,
    ///Whether the decoration is dense.
    #[serde(rename = "isDense", default, skip_serializing_if = "Option::is_none")]
    pub is_dense: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<LenraComponent>>,
    ///The style to use for the label.
    #[serde(rename = "labelStyle", default, skip_serializing_if = "Option::is_none")]
    pub label_style: Option<StylesTextStyle>,
    ///The text that describes the input field.
    #[serde(rename = "labelText", default, skip_serializing_if = "Option::is_none")]
    pub label_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<Box<LenraComponent>>,
    #[serde(rename = "prefixIcon", default, skip_serializing_if = "Option::is_none")]
    pub prefix_icon: Option<Icon>,
    ///the color of the prefixIcon
    #[serde(
        rename = "prefixIconColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_icon_color: Option<StylesColor>,
    ///The constraints for the prefixIcon.
    #[serde(
        rename = "prefixIconConstraints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_icon_constraints: Option<StylesBoxConstraints>,
    ///The style to use for the prefixText.
    #[serde(rename = "prefixStyle", default, skip_serializing_if = "Option::is_none")]
    pub prefix_style: Option<StylesTextStyle>,
    ///The text to display before the input.
    #[serde(rename = "prefixText", default, skip_serializing_if = "Option::is_none")]
    pub prefix_text: Option<String>,
    ///The semantic label for the counterText.
    #[serde(
        rename = "semanticCounterText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub semantic_counter_text: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub suffix: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "suffixIcon", default, skip_serializing_if = "Option::is_none")]
    pub suffix_icon: Option<Icon>,
    ///the color of the sufficIcon
    #[serde(
        rename = "suffixIconColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suffix_icon_color: Option<StylesColor>,
    ///The constraints for the suffixIcon.
    #[serde(
        rename = "suffixIconConstraints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suffix_icon_constraints: Option<StylesBoxConstraints>,
    ///The style to use for the suffixText.
    #[serde(rename = "suffixStyle", default, skip_serializing_if = "Option::is_none")]
    pub suffix_style: Option<StylesTextStyle>,
    ///The text to display after the input.
    #[serde(rename = "suffixText", default, skip_serializing_if = "Option::is_none")]
    pub suffix_text: Option<String>,
}
impl From<&StylesInputDecoration> for StylesInputDecoration {
    fn from(value: &StylesInputDecoration) -> Self {
        value.clone()
    }
}
impl StylesInputDecoration {
    pub fn builder() -> builder::StylesInputDecoration {
        builder::StylesInputDecoration::default()
    }
}
///Element of type locale
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesLocale {
    ///The region subtag for the locale.
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    ///The primary language subtag for the locale.
    #[serde(rename = "languageCode", default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    ///The script subtag for the locale.
    #[serde(rename = "scriptCode", default, skip_serializing_if = "Option::is_none")]
    pub script_code: Option<String>,
}
impl From<&StylesLocale> for StylesLocale {
    fn from(value: &StylesLocale) -> Self {
        value.clone()
    }
}
impl StylesLocale {
    pub fn builder() -> builder::StylesLocale {
        builder::StylesLocale::default()
    }
}
///Element of type MaterialTapTargetSize
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesMaterialTapTargetSize {
    #[serde(rename = "shrinkWrap")]
    ShrinkWrap,
    #[serde(rename = "padded")]
    Padded,
}
impl From<&StylesMaterialTapTargetSize> for StylesMaterialTapTargetSize {
    fn from(value: &StylesMaterialTapTargetSize) -> Self {
        value.clone()
    }
}
impl ToString for StylesMaterialTapTargetSize {
    fn to_string(&self) -> String {
        match *self {
            Self::ShrinkWrap => "shrinkWrap".to_string(),
            Self::Padded => "padded".to_string(),
        }
    }
}
impl std::str::FromStr for StylesMaterialTapTargetSize {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "shrinkWrap" => Ok(Self::ShrinkWrap),
            "padded" => Ok(Self::Padded),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesMaterialTapTargetSize {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesMaterialTapTargetSize {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesMaterialTapTargetSize {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type MaxLengthEnforcement.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesMaxLengthEnforcement {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "enforced")]
    Enforced,
    #[serde(rename = "truncateAfterCompositionEnds")]
    TruncateAfterCompositionEnds,
}
impl From<&StylesMaxLengthEnforcement> for StylesMaxLengthEnforcement {
    fn from(value: &StylesMaxLengthEnforcement) -> Self {
        value.clone()
    }
}
impl ToString for StylesMaxLengthEnforcement {
    fn to_string(&self) -> String {
        match *self {
            Self::None => "none".to_string(),
            Self::Enforced => "enforced".to_string(),
            Self::TruncateAfterCompositionEnds => {
                "truncateAfterCompositionEnds".to_string()
            }
        }
    }
}
impl std::str::FromStr for StylesMaxLengthEnforcement {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "none" => Ok(Self::None),
            "enforced" => Ok(Self::Enforced),
            "truncateAfterCompositionEnds" => Ok(Self::TruncateAfterCompositionEnds),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesMaxLengthEnforcement {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesMaxLengthEnforcement {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesMaxLengthEnforcement {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type Offset
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesOffset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
}
impl From<&StylesOffset> for StylesOffset {
    fn from(value: &StylesOffset) -> Self {
        value.clone()
    }
}
impl StylesOffset {
    pub fn builder() -> builder::StylesOffset {
        builder::StylesOffset::default()
    }
}
///Element of type OutlinedBorder
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesOutlinedBorder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<StylesBorderSide>,
}
impl From<&StylesOutlinedBorder> for StylesOutlinedBorder {
    fn from(value: &StylesOutlinedBorder) -> Self {
        value.clone()
    }
}
impl StylesOutlinedBorder {
    pub fn builder() -> builder::StylesOutlinedBorder {
        builder::StylesOutlinedBorder::default()
    }
}
///Element of type Padding
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesPadding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<f64>,
}
impl From<&StylesPadding> for StylesPadding {
    fn from(value: &StylesPadding) -> Self {
        value.clone()
    }
}
impl StylesPadding {
    pub fn builder() -> builder::StylesPadding {
        builder::StylesPadding::default()
    }
}
///Element of type RadioStyle
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesRadioStyle {
    ///Color of the active radio button
    #[serde(rename = "activeColor", default, skip_serializing_if = "Option::is_none")]
    pub active_color: Option<StylesColor>,
    ///Color of the radio when it is focused
    #[serde(rename = "focusColor", default, skip_serializing_if = "Option::is_none")]
    pub focus_color: Option<StylesColor>,
    ///Color when the mouse is over the element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovercolor: Option<StylesColor>,
    #[serde(rename = "splashRadius", default, skip_serializing_if = "Option::is_none")]
    pub splash_radius: Option<f64>,
    ///Color when the radio is not selected
    #[serde(
        rename = "unselectedColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unselected_color: Option<StylesColor>,
    #[serde(rename = "visualDensity", default, skip_serializing_if = "Option::is_none")]
    pub visual_density: Option<StylesVisualDensity>,
}
impl From<&StylesRadioStyle> for StylesRadioStyle {
    fn from(value: &StylesRadioStyle) -> Self {
        value.clone()
    }
}
impl StylesRadioStyle {
    pub fn builder() -> builder::StylesRadioStyle {
        builder::StylesRadioStyle::default()
    }
}
///Element of type Radius
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesRadius {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}
impl From<&StylesRadius> for StylesRadius {
    fn from(value: &StylesRadius) -> Self {
        value.clone()
    }
}
impl StylesRadius {
    pub fn builder() -> builder::StylesRadius {
        builder::StylesRadius::default()
    }
}
///Element of type Rect
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesRect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}
impl From<&StylesRect> for StylesRect {
    fn from(value: &StylesRect) -> Self {
        value.clone()
    }
}
impl StylesRect {
    pub fn builder() -> builder::StylesRect {
        builder::StylesRect::default()
    }
}
///The size to use, the component will be sized according to the value.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesSize {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}
impl From<&StylesSize> for StylesSize {
    fn from(value: &StylesSize) -> Self {
        value.clone()
    }
}
impl ToString for StylesSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
        }
    }
}
impl std::str::FromStr for StylesSize {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesSize {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesSize {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesSize {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type SliderStyle
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesSliderStyle {
    #[serde(rename = "activeColor", default, skip_serializing_if = "Option::is_none")]
    pub active_color: Option<StylesColor>,
    #[serde(rename = "inactiveColor", default, skip_serializing_if = "Option::is_none")]
    pub inactive_color: Option<StylesColor>,
    #[serde(rename = "thumbColor", default, skip_serializing_if = "Option::is_none")]
    pub thumb_color: Option<StylesColor>,
}
impl From<&StylesSliderStyle> for StylesSliderStyle {
    fn from(value: &StylesSliderStyle) -> Self {
        value.clone()
    }
}
impl StylesSliderStyle {
    pub fn builder() -> builder::StylesSliderStyle {
        builder::StylesSliderStyle::default()
    }
}
///The StackFit enum.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesStackFit {
    #[serde(rename = "expand")]
    Expand,
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "passthrough")]
    Passthrough,
}
impl From<&StylesStackFit> for StylesStackFit {
    fn from(value: &StylesStackFit) -> Self {
        value.clone()
    }
}
impl ToString for StylesStackFit {
    fn to_string(&self) -> String {
        match *self {
            Self::Expand => "expand".to_string(),
            Self::Loose => "loose".to_string(),
            Self::Passthrough => "passthrough".to_string(),
        }
    }
}
impl std::str::FromStr for StylesStackFit {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "expand" => Ok(Self::Expand),
            "loose" => Ok(Self::Loose),
            "passthrough" => Ok(Self::Passthrough),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesStackFit {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesStackFit {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesStackFit {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Defines the strut of a text line.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesStrutStyle {
    ///A label to help identify this strut style.
    #[serde(rename = "debugLabel", default, skip_serializing_if = "Option::is_none")]
    pub debug_label: Option<String>,
    ///The font family to use for this strut style.
    #[serde(rename = "fontFamily", default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    ///A list of fallback font families to use for this strut style.
    #[serde(
        rename = "fontFamilyFallback",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub font_family_fallback: Vec<serde_json::Value>,
    #[serde(rename = "fontSize", default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    ///The font weight to use for this strut style.
    #[serde(rename = "fontWeight", default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    ///Whether to force the strut height.
    #[serde(
        rename = "forceStrutHeight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_strut_height: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leading: Option<f64>,
    #[serde(
        rename = "leadingDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub leading_distribution: Option<StylesTextLeadingDistribution>,
}
impl From<&StylesStrutStyle> for StylesStrutStyle {
    fn from(value: &StylesStrutStyle) -> Self {
        value.clone()
    }
}
impl StylesStrutStyle {
    pub fn builder() -> builder::StylesStrutStyle {
        builder::StylesStrutStyle::default()
    }
}
///The style to use, the component will be changed according to the theme.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesStyle {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "tertiary")]
    Tertiary,
}
impl From<&StylesStyle> for StylesStyle {
    fn from(value: &StylesStyle) -> Self {
        value.clone()
    }
}
impl ToString for StylesStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::Primary => "primary".to_string(),
            Self::Secondary => "secondary".to_string(),
            Self::Tertiary => "tertiary".to_string(),
        }
    }
}
impl std::str::FromStr for StylesStyle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "primary" => Ok(Self::Primary),
            "secondary" => Ok(Self::Secondary),
            "tertiary" => Ok(Self::Tertiary),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesStyle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesStyle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesStyle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type TextAlign.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextAlign {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "justify")]
    Justify,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
}
impl From<&StylesTextAlign> for StylesTextAlign {
    fn from(value: &StylesTextAlign) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextAlign {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "left".to_string(),
            Self::Right => "right".to_string(),
            Self::Center => "center".to_string(),
            Self::Justify => "justify".to_string(),
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextAlign {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "center" => Ok(Self::Center),
            "justify" => Ok(Self::Justify),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextAlign {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextAlign {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextAlign {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type TextAlignVertical.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextAlignVertical {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "top")]
    Top,
}
impl From<&StylesTextAlignVertical> for StylesTextAlignVertical {
    fn from(value: &StylesTextAlignVertical) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextAlignVertical {
    fn to_string(&self) -> String {
        match *self {
            Self::Bottom => "bottom".to_string(),
            Self::Center => "center".to_string(),
            Self::Top => "top".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextAlignVertical {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "bottom" => Ok(Self::Bottom),
            "center" => Ok(Self::Center),
            "top" => Ok(Self::Top),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextAlignVertical {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextAlignVertical {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextAlignVertical {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///A horizontal line used for aligning text.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextBaseline {
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "ideographic")]
    Ideographic,
}
impl From<&StylesTextBaseline> for StylesTextBaseline {
    fn from(value: &StylesTextBaseline) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextBaseline {
    fn to_string(&self) -> String {
        match *self {
            Self::Alphabetic => "alphabetic".to_string(),
            Self::Ideographic => "ideographic".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextBaseline {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "alphabetic" => Ok(Self::Alphabetic),
            "ideographic" => Ok(Self::Ideographic),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextBaseline {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextBaseline {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextBaseline {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Component of type TextCapitalization.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextCapitalization {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "words")]
    Words,
    #[serde(rename = "sentences")]
    Sentences,
    #[serde(rename = "characters")]
    Characters,
}
impl From<&StylesTextCapitalization> for StylesTextCapitalization {
    fn from(value: &StylesTextCapitalization) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextCapitalization {
    fn to_string(&self) -> String {
        match *self {
            Self::None => "none".to_string(),
            Self::Words => "words".to_string(),
            Self::Sentences => "sentences".to_string(),
            Self::Characters => "characters".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextCapitalization {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "none" => Ok(Self::None),
            "words" => Ok(Self::Words),
            "sentences" => Ok(Self::Sentences),
            "characters" => Ok(Self::Characters),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextCapitalization {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextCapitalization {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextCapitalization {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Allows you to underline, overline or strike out the text.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextDecoration {
    #[serde(rename = "lineThrough")]
    LineThrough,
    #[serde(rename = "overline")]
    Overline,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "none")]
    None,
}
impl From<&StylesTextDecoration> for StylesTextDecoration {
    fn from(value: &StylesTextDecoration) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextDecoration {
    fn to_string(&self) -> String {
        match *self {
            Self::LineThrough => "lineThrough".to_string(),
            Self::Overline => "overline".to_string(),
            Self::Underline => "underline".to_string(),
            Self::None => "none".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextDecoration {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "lineThrough" => Ok(Self::LineThrough),
            "overline" => Ok(Self::Overline),
            "underline" => Ok(Self::Underline),
            "none" => Ok(Self::None),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextDecoration {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextDecoration {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextDecoration {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The style in which to draw a text decoration.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextDecorationStyle {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "wavy")]
    Wavy,
}
impl From<&StylesTextDecorationStyle> for StylesTextDecorationStyle {
    fn from(value: &StylesTextDecorationStyle) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextDecorationStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::Dashed => "dashed".to_string(),
            Self::Dotted => "dotted".to_string(),
            Self::Double => "double".to_string(),
            Self::Solid => "solid".to_string(),
            Self::Wavy => "wavy".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextDecorationStyle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "dashed" => Ok(Self::Dashed),
            "dotted" => Ok(Self::Dotted),
            "double" => Ok(Self::Double),
            "solid" => Ok(Self::Solid),
            "wavy" => Ok(Self::Wavy),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextDecorationStyle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextDecorationStyle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextDecorationStyle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///In which direction the elements should be placed following the horizontal axis.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextDirection {
    #[serde(rename = "ltr")]
    Ltr,
    #[serde(rename = "rtl")]
    Rtl,
}
impl From<&StylesTextDirection> for StylesTextDirection {
    fn from(value: &StylesTextDirection) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextDirection {
    fn to_string(&self) -> String {
        match *self {
            Self::Ltr => "ltr".to_string(),
            Self::Rtl => "rtl".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextDirection {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "ltr" => Ok(Self::Ltr),
            "rtl" => Ok(Self::Rtl),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextDirection {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextDirection {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextDirection {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type TextFieldStyle
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesTextFieldStyle {
    ///The color of the cursor.
    #[serde(rename = "cursorColor", default, skip_serializing_if = "Option::is_none")]
    pub cursor_color: Option<StylesColor>,
    #[serde(rename = "cursorHeight", default, skip_serializing_if = "Option::is_none")]
    pub cursor_height: Option<f64>,
    ///The radius of the cursor.
    #[serde(rename = "cursorRadius", default, skip_serializing_if = "Option::is_none")]
    pub cursor_radius: Option<StylesRadius>,
    #[serde(rename = "cursorWidth", default, skip_serializing_if = "Option::is_none")]
    pub cursor_width: Option<f64>,
    ///The decoration of the input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decoration: Option<StylesInputDecoration>,
    ///The appearance of the keyboard.
    #[serde(
        rename = "keyboardAppearance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keyboard_appearance: Option<StylesBrightness>,
    ///The character used to obscure the text.
    #[serde(
        rename = "obscuringCharacter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub obscuring_character: Option<String>,
    ///The padding of the scrollable when the Textfield scrolls into view.
    #[serde(rename = "scrollPadding", default, skip_serializing_if = "Option::is_none")]
    pub scroll_padding: Option<StylesPadding>,
    ///The height of the selection highlight boxes.
    #[serde(
        rename = "selectionHeightStyle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selection_height_style: Option<StylesBoxHeightStyle>,
    ///The width of the selection highlight boxes.
    #[serde(
        rename = "selectionWidthStyle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selection_width_style: Option<StylesBoxWidthStyle>,
    #[serde(rename = "strutStyle", default, skip_serializing_if = "Option::is_none")]
    pub strut_style: Option<StylesStrutStyle>,
    ///The alignment of the text.
    #[serde(rename = "textAlign", default, skip_serializing_if = "Option::is_none")]
    pub text_align: Option<StylesTextAlign>,
    ///How the text should be aligned vertically.
    #[serde(
        rename = "textAlignVertical",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub text_align_vertical: Option<StylesTextAlignVertical>,
    ///The style of the text.
    #[serde(rename = "textStyle", default, skip_serializing_if = "Option::is_none")]
    pub text_style: Option<StylesTextStyle>,
}
impl From<&StylesTextFieldStyle> for StylesTextFieldStyle {
    fn from(value: &StylesTextFieldStyle) -> Self {
        value.clone()
    }
}
impl StylesTextFieldStyle {
    pub fn builder() -> builder::StylesTextFieldStyle {
        builder::StylesTextFieldStyle::default()
    }
}
///Component of type TextInputAction.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextInputAction {
    #[serde(rename = "continueAction")]
    ContinueAction,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "emergencyCall")]
    EmergencyCall,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "newline")]
    Newline,
    #[serde(rename = "next")]
    Next,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "previous")]
    Previous,
    #[serde(rename = "route")]
    Route,
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "unspecified")]
    Unspecified,
}
impl From<&StylesTextInputAction> for StylesTextInputAction {
    fn from(value: &StylesTextInputAction) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextInputAction {
    fn to_string(&self) -> String {
        match *self {
            Self::ContinueAction => "continueAction".to_string(),
            Self::Done => "done".to_string(),
            Self::EmergencyCall => "emergencyCall".to_string(),
            Self::Go => "go".to_string(),
            Self::Join => "join".to_string(),
            Self::Newline => "newline".to_string(),
            Self::Next => "next".to_string(),
            Self::None => "none".to_string(),
            Self::Previous => "previous".to_string(),
            Self::Route => "route".to_string(),
            Self::Search => "search".to_string(),
            Self::Send => "send".to_string(),
            Self::Unspecified => "unspecified".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextInputAction {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "continueAction" => Ok(Self::ContinueAction),
            "done" => Ok(Self::Done),
            "emergencyCall" => Ok(Self::EmergencyCall),
            "go" => Ok(Self::Go),
            "join" => Ok(Self::Join),
            "newline" => Ok(Self::Newline),
            "next" => Ok(Self::Next),
            "none" => Ok(Self::None),
            "previous" => Ok(Self::Previous),
            "route" => Ok(Self::Route),
            "search" => Ok(Self::Search),
            "send" => Ok(Self::Send),
            "unspecified" => Ok(Self::Unspecified),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextInputAction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextInputAction {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextInputAction {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of textInput Type
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesTextInputType {
    ///Whether to show copy option in toolbar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy: Option<bool>,
    ///Whether to show cut option in toolbar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cut: Option<bool>,
    ///Whether to show past option in toolbar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paste: Option<bool>,
    ///Whether to show select all option in toolbar
    #[serde(rename = "selectAll", default, skip_serializing_if = "Option::is_none")]
    pub select_all: Option<bool>,
}
impl From<&StylesTextInputType> for StylesTextInputType {
    fn from(value: &StylesTextInputType) -> Self {
        value.clone()
    }
}
impl StylesTextInputType {
    pub fn builder() -> builder::StylesTextInputType {
        builder::StylesTextInputType::default()
    }
}
///The TextLeadingDistribution enum.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextLeadingDistribution {
    #[serde(rename = "even")]
    Even,
    #[serde(rename = "proportional")]
    Proportional,
}
impl From<&StylesTextLeadingDistribution> for StylesTextLeadingDistribution {
    fn from(value: &StylesTextLeadingDistribution) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextLeadingDistribution {
    fn to_string(&self) -> String {
        match *self {
            Self::Even => "even".to_string(),
            Self::Proportional => "proportional".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextLeadingDistribution {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "even" => Ok(Self::Even),
            "proportional" => Ok(Self::Proportional),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextLeadingDistribution {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextLeadingDistribution {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextLeadingDistribution {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The style of the Text.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesTextStyle {
    ///The color of the text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<StylesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decoration: Option<StylesTextDecoration>,
    ///The color of the decoration.
    #[serde(
        rename = "decorationColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decoration_color: Option<StylesColor>,
    #[serde(
        rename = "decorationStyle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decoration_style: Option<StylesTextDecorationStyle>,
    #[serde(
        rename = "decorationThickness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decoration_thickness: Option<f64>,
    ///The font family of the text.
    #[serde(rename = "fontFamily", default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    ///The list of font families to use if the first font family could not be found.
    #[serde(
        rename = "fontFamilyFallback",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub font_family_fallback: Vec<String>,
    #[serde(rename = "fontSize", default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    ///The style of the text.
    #[serde(rename = "fontStyle", default, skip_serializing_if = "Option::is_none")]
    pub font_style: Option<StylesTextStyleFontStyle>,
    ///The weight of the text.
    #[serde(rename = "fontWeight", default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<StylesTextStyleFontWeight>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(rename = "letterSpacing", default, skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<f64>,
    ///How visual text overflow should be handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overflow: Option<StylesTextStyleOverflow>,
    ///A list of Shadows that will be painted underneath the text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shadows: Vec<StylesBoxShadow>,
    ///The common baseline that should be aligned between this text and its parent text.
    #[serde(rename = "textBaseline", default, skip_serializing_if = "Option::is_none")]
    pub text_baseline: Option<StylesTextBaseline>,
    #[serde(rename = "wordSpacing", default, skip_serializing_if = "Option::is_none")]
    pub word_spacing: Option<f64>,
}
impl From<&StylesTextStyle> for StylesTextStyle {
    fn from(value: &StylesTextStyle) -> Self {
        value.clone()
    }
}
impl StylesTextStyle {
    pub fn builder() -> builder::StylesTextStyle {
        builder::StylesTextStyle::default()
    }
}
///The style of the text.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextStyleFontStyle {
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "normal")]
    Normal,
}
impl From<&StylesTextStyleFontStyle> for StylesTextStyleFontStyle {
    fn from(value: &StylesTextStyleFontStyle) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextStyleFontStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::Italic => "italic".to_string(),
            Self::Normal => "normal".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextStyleFontStyle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "italic" => Ok(Self::Italic),
            "normal" => Ok(Self::Normal),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextStyleFontStyle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextStyleFontStyle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextStyleFontStyle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The weight of the text.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextStyleFontWeight {
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "w100")]
    W100,
    #[serde(rename = "w200")]
    W200,
    #[serde(rename = "w300")]
    W300,
    #[serde(rename = "w400")]
    W400,
    #[serde(rename = "w500")]
    W500,
    #[serde(rename = "w600")]
    W600,
    #[serde(rename = "w700")]
    W700,
    #[serde(rename = "w800")]
    W800,
    #[serde(rename = "w900")]
    W900,
}
impl From<&StylesTextStyleFontWeight> for StylesTextStyleFontWeight {
    fn from(value: &StylesTextStyleFontWeight) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextStyleFontWeight {
    fn to_string(&self) -> String {
        match *self {
            Self::Bold => "bold".to_string(),
            Self::Normal => "normal".to_string(),
            Self::W100 => "w100".to_string(),
            Self::W200 => "w200".to_string(),
            Self::W300 => "w300".to_string(),
            Self::W400 => "w400".to_string(),
            Self::W500 => "w500".to_string(),
            Self::W600 => "w600".to_string(),
            Self::W700 => "w700".to_string(),
            Self::W800 => "w800".to_string(),
            Self::W900 => "w900".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextStyleFontWeight {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "bold" => Ok(Self::Bold),
            "normal" => Ok(Self::Normal),
            "w100" => Ok(Self::W100),
            "w200" => Ok(Self::W200),
            "w300" => Ok(Self::W300),
            "w400" => Ok(Self::W400),
            "w500" => Ok(Self::W500),
            "w600" => Ok(Self::W600),
            "w700" => Ok(Self::W700),
            "w800" => Ok(Self::W800),
            "w900" => Ok(Self::W900),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextStyleFontWeight {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextStyleFontWeight {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextStyleFontWeight {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///How visual text overflow should be handled.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesTextStyleOverflow {
    #[serde(rename = "clip")]
    Clip,
    #[serde(rename = "ellipsis")]
    Ellipsis,
    #[serde(rename = "fade")]
    Fade,
    #[serde(rename = "visible")]
    Visible,
}
impl From<&StylesTextStyleOverflow> for StylesTextStyleOverflow {
    fn from(value: &StylesTextStyleOverflow) -> Self {
        value.clone()
    }
}
impl ToString for StylesTextStyleOverflow {
    fn to_string(&self) -> String {
        match *self {
            Self::Clip => "clip".to_string(),
            Self::Ellipsis => "ellipsis".to_string(),
            Self::Fade => "fade".to_string(),
            Self::Visible => "visible".to_string(),
        }
    }
}
impl std::str::FromStr for StylesTextStyleOverflow {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "clip" => Ok(Self::Clip),
            "ellipsis" => Ok(Self::Ellipsis),
            "fade" => Ok(Self::Fade),
            "visible" => Ok(Self::Visible),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesTextStyleOverflow {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesTextStyleOverflow {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesTextStyleOverflow {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type ToggleStyle
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesToggleStyle {
    #[serde(rename = "activeColor", default, skip_serializing_if = "Option::is_none")]
    pub active_color: Option<StylesColor>,
    #[serde(
        rename = "activeThumbImage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_thumb_image: Option<Image>,
    #[serde(
        rename = "activeTrackColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_track_color: Option<StylesColor>,
    #[serde(rename = "focusColor", default, skip_serializing_if = "Option::is_none")]
    pub focus_color: Option<StylesColor>,
    #[serde(rename = "hoverColor", default, skip_serializing_if = "Option::is_none")]
    pub hover_color: Option<StylesColor>,
    #[serde(
        rename = "inactiveThumbColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inactive_thumb_color: Option<StylesColor>,
    #[serde(
        rename = "inactiveThumbImage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inactive_thumb_image: Option<Image>,
    #[serde(
        rename = "inactiveTrackColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inactive_track_color: Option<StylesColor>,
    #[serde(
        rename = "materialTapTargetSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub material_tap_target_size: Option<StylesToggleStyleMaterialTapTargetSize>,
}
impl From<&StylesToggleStyle> for StylesToggleStyle {
    fn from(value: &StylesToggleStyle) -> Self {
        value.clone()
    }
}
impl StylesToggleStyle {
    pub fn builder() -> builder::StylesToggleStyle {
        builder::StylesToggleStyle::default()
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesToggleStyleMaterialTapTargetSize {
    #[serde(rename = "padded")]
    Padded,
    #[serde(rename = "shrinkWrap")]
    ShrinkWrap,
}
impl From<&StylesToggleStyleMaterialTapTargetSize>
for StylesToggleStyleMaterialTapTargetSize {
    fn from(value: &StylesToggleStyleMaterialTapTargetSize) -> Self {
        value.clone()
    }
}
impl ToString for StylesToggleStyleMaterialTapTargetSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Padded => "padded".to_string(),
            Self::ShrinkWrap => "shrinkWrap".to_string(),
        }
    }
}
impl std::str::FromStr for StylesToggleStyleMaterialTapTargetSize {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "padded" => Ok(Self::Padded),
            "shrinkWrap" => Ok(Self::ShrinkWrap),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesToggleStyleMaterialTapTargetSize {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesToggleStyleMaterialTapTargetSize {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesToggleStyleMaterialTapTargetSize {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type toolbar options
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StylesToolbarOptions {
    ///The number is decimal, allowing a decimal point to provide fractional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal: Option<bool>,
    ///The number is signed, allowing a positive or negative sign at the start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
}
impl From<&StylesToolbarOptions> for StylesToolbarOptions {
    fn from(value: &StylesToolbarOptions) -> Self {
        value.clone()
    }
}
impl StylesToolbarOptions {
    pub fn builder() -> builder::StylesToolbarOptions {
        builder::StylesToolbarOptions::default()
    }
}
///How the objects should be aligned following the vertical axis.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesVerticalDirection {
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "up")]
    Up,
}
impl From<&StylesVerticalDirection> for StylesVerticalDirection {
    fn from(value: &StylesVerticalDirection) -> Self {
        value.clone()
    }
}
impl ToString for StylesVerticalDirection {
    fn to_string(&self) -> String {
        match *self {
            Self::Down => "down".to_string(),
            Self::Up => "up".to_string(),
        }
    }
}
impl std::str::FromStr for StylesVerticalDirection {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesVerticalDirection {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesVerticalDirection {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesVerticalDirection {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///The visual density of UI components.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesVisualDensity {
    #[serde(rename = "comfortable")]
    Comfortable,
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "standard")]
    Standard,
}
impl From<&StylesVisualDensity> for StylesVisualDensity {
    fn from(value: &StylesVisualDensity) -> Self {
        value.clone()
    }
}
impl ToString for StylesVisualDensity {
    fn to_string(&self) -> String {
        match *self {
            Self::Comfortable => "comfortable".to_string(),
            Self::Compact => "compact".to_string(),
            Self::Standard => "standard".to_string(),
        }
    }
}
impl std::str::FromStr for StylesVisualDensity {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "comfortable" => Ok(Self::Comfortable),
            "compact" => Ok(Self::Compact),
            "standard" => Ok(Self::Standard),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesVisualDensity {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesVisualDensity {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesVisualDensity {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///How the objects in the Wrap should be aligned.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesWrapAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "spaceBetween")]
    SpaceBetween,
    #[serde(rename = "spaceAround")]
    SpaceAround,
    #[serde(rename = "spaceEvenly")]
    SpaceEvenly,
}
impl From<&StylesWrapAlignment> for StylesWrapAlignment {
    fn from(value: &StylesWrapAlignment) -> Self {
        value.clone()
    }
}
impl ToString for StylesWrapAlignment {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
            Self::Center => "center".to_string(),
            Self::SpaceBetween => "spaceBetween".to_string(),
            Self::SpaceAround => "spaceAround".to_string(),
            Self::SpaceEvenly => "spaceEvenly".to_string(),
        }
    }
}
impl std::str::FromStr for StylesWrapAlignment {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "spaceBetween" => Ok(Self::SpaceBetween),
            "spaceAround" => Ok(Self::SpaceAround),
            "spaceEvenly" => Ok(Self::SpaceEvenly),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesWrapAlignment {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesWrapAlignment {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesWrapAlignment {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///How the objects in the Wrap should be aligned on the CrossAxis.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum StylesWrapCrossAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
}
impl From<&StylesWrapCrossAlignment> for StylesWrapCrossAlignment {
    fn from(value: &StylesWrapCrossAlignment) -> Self {
        value.clone()
    }
}
impl ToString for StylesWrapCrossAlignment {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
            Self::Center => "center".to_string(),
        }
    }
}
impl std::str::FromStr for StylesWrapCrossAlignment {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StylesWrapCrossAlignment {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StylesWrapCrossAlignment {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StylesWrapCrossAlignment {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type Text
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    ///Additional texts to add after this text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<StylesLocale>,
    ///The value to explain a different semantics
    #[serde(rename = "semanticsLabel", default, skip_serializing_if = "Option::is_none")]
    pub semantics_label: Option<String>,
    ///Whether the assistive technologies should spell out this text character by character
    #[serde(rename = "spellOut", default, skip_serializing_if = "Option::is_none")]
    pub spell_out: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StylesTextStyle>,
    ///The text alignment
    #[serde(rename = "textAlign", default, skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextTextAlign>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    ///the value displayed in the element
    pub value: String,
}
impl From<&Text> for Text {
    fn from(value: &Text) -> Self {
        value.clone()
    }
}
impl Text {
    pub fn builder() -> builder::Text {
        builder::Text::default()
    }
}
///The text alignment
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum TextTextAlign {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "justify")]
    Justify,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
}
impl From<&TextTextAlign> for TextTextAlign {
    fn from(value: &TextTextAlign) -> Self {
        value.clone()
    }
}
impl ToString for TextTextAlign {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "left".to_string(),
            Self::Center => "center".to_string(),
            Self::Right => "right".to_string(),
            Self::Justify => "justify".to_string(),
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
        }
    }
}
impl std::str::FromStr for TextTextAlign {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            "justify" => Ok(Self::Justify),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TextTextAlign {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextTextAlign {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextTextAlign {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type TextField
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Textfield {
    ///Whether to enable the autocorrection
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autocorrect: Option<bool>,
    ///The type of this text input to provide autofill hints.
    #[serde(rename = "autofillHints", default, skip_serializing_if = "Option::is_none")]
    pub autofill_hints: Option<StylesAutofillHints>,
    ///Whether this Textfield should be focused initially.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autofocus: Option<bool>,
    ///Callback that generates a custom counter view.
    #[serde(rename = "buildCounter", default, skip_serializing_if = "Option::is_none")]
    pub build_counter: Option<Listener>,
    ///Determines the way that drag start behavior is handled.
    #[serde(
        rename = "dragStartBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drag_start_behavior: Option<StylesDragStartBehavior>,
    ///Whether to enable user interface options to change the text selection.
    #[serde(
        rename = "enableInteractiveSelection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_interactive_selection: Option<bool>,
    ///Whether the text field is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///Whether the TextField is sized to fill its parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<bool>,
    ///The type of the keyboard to use for editing the text.
    #[serde(rename = "keyboardType", default, skip_serializing_if = "Option::is_none")]
    pub keyboard_type: Option<StylesTextInputType>,
    ///The maximum number of characters to allow in the text field.
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    ///Determines how the maxLength limit should be enforced.
    #[serde(
        rename = "maxLengthEnforcement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_length_enforcement: Option<StylesMaxLengthEnforcement>,
    ///The maximum number of lines to show at one time.
    #[serde(rename = "maxLines", default, skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<i64>,
    ///The minimum number of lines to occupy on the screen.
    #[serde(rename = "minLines", default, skip_serializing_if = "Option::is_none")]
    pub min_lines: Option<i64>,
    ///The name that will be used in the form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether to hide the text being edited.
    #[serde(rename = "obscureText", default, skip_serializing_if = "Option::is_none")]
    pub obscure_text: Option<bool>,
    ///This is used to receive a private command from the input method.
    #[serde(
        rename = "onAppPrivateCommand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_app_private_command: Option<Listener>,
    ///Callback when the user changes the text field value.
    #[serde(rename = "onChanged", default, skip_serializing_if = "Option::is_none")]
    pub on_changed: Option<Listener>,
    ///Callback when the user finishes editing the text field.
    #[serde(
        rename = "onEditingComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_editing_complete: Option<Listener>,
    ///Callback when the user tells he is done editing the text field.
    #[serde(rename = "onSubmitted", default, skip_serializing_if = "Option::is_none")]
    pub on_submitted: Option<Listener>,
    ///Callback when the user taps on the text field.
    #[serde(rename = "onTap", default, skip_serializing_if = "Option::is_none")]
    pub on_tap: Option<Listener>,
    ///Whether the text can be changed.
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    ///Whether to show the cursor.
    #[serde(rename = "showCursor", default, skip_serializing_if = "Option::is_none")]
    pub show_cursor: Option<bool>,
    ///The style of the Textfield.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StylesTextFieldStyle>,
    ///Configures how the platform keyboard will select an uppercase or lowercase keyboard.
    #[serde(
        rename = "textCapitalization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub text_capitalization: Option<StylesTextCapitalization>,
    ///The direction of the text.
    #[serde(rename = "textDirection", default, skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<StylesTextDirection>,
    ///The type of the action button to use for the keyboard.
    #[serde(
        rename = "textInputAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub text_input_action: Option<StylesTextInputAction>,
    ///Configuration of toolbar options
    #[serde(rename = "toolbarOptions", default, skip_serializing_if = "Option::is_none")]
    pub toolbar_options: Option<StylesToolbarOptions>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    ///The value displayed inside the Textfield
    pub value: String,
}
impl From<&Textfield> for Textfield {
    fn from(value: &Textfield) -> Self {
        value.clone()
    }
}
impl Textfield {
    pub fn builder() -> builder::Textfield {
        builder::Textfield::default()
    }
}
///Element of type Toggle
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Toggle {
    ///The default focus in boolean.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autofocus: Option<bool>,
    ///The toggle is disabled if true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    ///Determines the way that drag start behavior is handled.
    #[serde(
        rename = "dragStartBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drag_start_behavior: Option<ToggleDragStartBehavior>,
    ///The name that will be used in the form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "onPressed", default, skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Listener>,
    #[serde(rename = "splashRadius", default, skip_serializing_if = "Option::is_none")]
    pub splash_radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StylesToggleStyle>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    ///The value of the element.
    pub value: bool,
}
impl From<&Toggle> for Toggle {
    fn from(value: &Toggle) -> Self {
        value.clone()
    }
}
impl Toggle {
    pub fn builder() -> builder::Toggle {
        builder::Toggle::default()
    }
}
///Determines the way that drag start behavior is handled.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum ToggleDragStartBehavior {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "down")]
    Down,
}
impl From<&ToggleDragStartBehavior> for ToggleDragStartBehavior {
    fn from(value: &ToggleDragStartBehavior) -> Self {
        value.clone()
    }
}
impl ToString for ToggleDragStartBehavior {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::Down => "down".to_string(),
        }
    }
}
impl std::str::FromStr for ToggleDragStartBehavior {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "down" => Ok(Self::Down),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ToggleDragStartBehavior {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ToggleDragStartBehavior {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ToggleDragStartBehavior {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Element of type view
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    ///The context projection. This field represents the projection of the context, allowing selective retrieval of specific elements. It is a map that specifies the desired elements to be included in the projection.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub context: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find: Option<ViewDefinitionsFind>,
    ///The name of the view
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<DefsProps>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
}
impl From<&View> for View {
    fn from(value: &View) -> Self {
        value.clone()
    }
}
impl View {
    pub fn builder() -> builder::View {
        builder::View::default()
    }
}
///Find query for view components
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ViewDefinitionsFind {
    ///the collection where the query is applied
    pub coll: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<DataProjection>,
    pub query: DataQuery,
}
impl From<&ViewDefinitionsFind> for ViewDefinitionsFind {
    fn from(value: &ViewDefinitionsFind) -> Self {
        value.clone()
    }
}
impl ViewDefinitionsFind {
    pub fn builder() -> builder::ViewDefinitionsFind {
        builder::ViewDefinitionsFind::default()
    }
}
///Element of type Wrap
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Wrap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<StylesWrapAlignment>,
    ///The children of the wrap.
    pub children: Vec<LenraComponent>,
    #[serde(
        rename = "crossAxisAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_axis_alignment: Option<StylesWrapCrossAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<StylesDirection>,
    #[serde(
        rename = "horizontalDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_direction: Option<StylesTextDirection>,
    #[serde(rename = "runAlignment", default, skip_serializing_if = "Option::is_none")]
    pub run_alignment: Option<StylesWrapAlignment>,
    #[serde(rename = "runSpacing", default, skip_serializing_if = "Option::is_none")]
    pub run_spacing: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f64>,
    ///The identifier of the component
    #[serde(rename = "_type")]
    pub type_: serde_json::Value,
    #[serde(
        rename = "verticalDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_direction: Option<StylesVerticalDirection>,
}
impl From<&Wrap> for Wrap {
    fn from(value: &Wrap) -> Self {
        value.clone()
    }
}
impl Wrap {
    pub fn builder() -> builder::Wrap {
        builder::Wrap::default()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Actionable {
        child: Result<super::LenraComponent, String>,
        on_double_pressed: Result<Option<super::Listener>, String>,
        on_hovered: Result<Option<super::Listener>, String>,
        on_long_pressed: Result<Option<super::Listener>, String>,
        on_pressed: Result<Option<super::Listener>, String>,
        on_pressed_cancel: Result<Option<super::Listener>, String>,
        submit: Result<Option<bool>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Actionable {
        fn default() -> Self {
            Self {
                child: Err("no value supplied for child".to_string()),
                on_double_pressed: Ok(Default::default()),
                on_hovered: Ok(Default::default()),
                on_long_pressed: Ok(Default::default()),
                on_pressed: Ok(Default::default()),
                on_pressed_cancel: Ok(Default::default()),
                submit: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Actionable {
        pub fn child<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LenraComponent>,
            T::Error: std::fmt::Display,
        {
            self
                .child = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for child: {}", e)
                });
            self
        }
        pub fn on_double_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_double_pressed = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for on_double_pressed: {}", e
                    )
                });
            self
        }
        pub fn on_hovered<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_hovered = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_hovered: {}", e)
                });
            self
        }
        pub fn on_long_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_long_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_long_pressed: {}", e)
                });
            self
        }
        pub fn on_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_pressed: {}", e)
                });
            self
        }
        pub fn on_pressed_cancel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed_cancel = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for on_pressed_cancel: {}", e
                    )
                });
            self
        }
        pub fn submit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .submit = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for submit: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Actionable> for super::Actionable {
        type Error = String;
        fn try_from(value: Actionable) -> Result<Self, String> {
            Ok(Self {
                child: value.child?,
                on_double_pressed: value.on_double_pressed?,
                on_hovered: value.on_hovered?,
                on_long_pressed: value.on_long_pressed?,
                on_pressed: value.on_pressed?,
                on_pressed_cancel: value.on_pressed_cancel?,
                submit: value.submit?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Actionable> for Actionable {
        fn from(value: super::Actionable) -> Self {
            Self {
                child: Ok(value.child),
                on_double_pressed: Ok(value.on_double_pressed),
                on_hovered: Ok(value.on_hovered),
                on_long_pressed: Ok(value.on_long_pressed),
                on_pressed: Ok(value.on_pressed),
                on_pressed_cancel: Ok(value.on_pressed_cancel),
                submit: Ok(value.submit),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Button {
        disabled: Result<Option<bool>, String>,
        left_icon: Result<Option<super::Icon>, String>,
        main_style: Result<Option<super::StylesStyle>, String>,
        on_pressed: Result<Option<super::Listener>, String>,
        right_icon: Result<Option<super::Icon>, String>,
        size: Result<Option<super::StylesSize>, String>,
        submit: Result<Option<bool>, String>,
        text: Result<String, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Button {
        fn default() -> Self {
            Self {
                disabled: Ok(Default::default()),
                left_icon: Ok(Default::default()),
                main_style: Ok(Default::default()),
                on_pressed: Ok(Default::default()),
                right_icon: Ok(Default::default()),
                size: Ok(Default::default()),
                submit: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Button {
        pub fn disabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .disabled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for disabled: {}", e)
                });
            self
        }
        pub fn left_icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .left_icon = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for left_icon: {}", e)
                });
            self
        }
        pub fn main_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .main_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for main_style: {}", e)
                });
            self
        }
        pub fn on_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_pressed: {}", e)
                });
            self
        }
        pub fn right_icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .right_icon = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for right_icon: {}", e)
                });
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesSize>>,
            T::Error: std::fmt::Display,
        {
            self
                .size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn submit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .submit = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for submit: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Button> for super::Button {
        type Error = String;
        fn try_from(value: Button) -> Result<Self, String> {
            Ok(Self {
                disabled: value.disabled?,
                left_icon: value.left_icon?,
                main_style: value.main_style?,
                on_pressed: value.on_pressed?,
                right_icon: value.right_icon?,
                size: value.size?,
                submit: value.submit?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Button> for Button {
        fn from(value: super::Button) -> Self {
            Self {
                disabled: Ok(value.disabled),
                left_icon: Ok(value.left_icon),
                main_style: Ok(value.main_style),
                on_pressed: Ok(value.on_pressed),
                right_icon: Ok(value.right_icon),
                size: Ok(value.size),
                submit: Ok(value.submit),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Carousel {
        children: Result<Vec<super::LenraComponent>, String>,
        options: Result<Option<super::StylesCarouselOptions>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Carousel {
        fn default() -> Self {
            Self {
                children: Err("no value supplied for children".to_string()),
                options: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Carousel {
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .children = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for children: {}", e)
                });
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesCarouselOptions>>,
            T::Error: std::fmt::Display,
        {
            self
                .options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for options: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Carousel> for super::Carousel {
        type Error = String;
        fn try_from(value: Carousel) -> Result<Self, String> {
            Ok(Self {
                children: value.children?,
                options: value.options?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Carousel> for Carousel {
        fn from(value: super::Carousel) -> Self {
            Self {
                children: Ok(value.children),
                options: Ok(value.options),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Checkbox {
        autofocus: Result<Option<bool>, String>,
        material_tap_target_size: Result<
            Option<super::StylesMaterialTapTargetSize>,
            String,
        >,
        name: Result<Option<String>, String>,
        on_pressed: Result<Option<super::Listener>, String>,
        style: Result<Option<super::StylesCheckboxStyle>, String>,
        tristate: Result<Option<bool>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<bool, String>,
    }
    impl Default for Checkbox {
        fn default() -> Self {
            Self {
                autofocus: Ok(Default::default()),
                material_tap_target_size: Ok(Default::default()),
                name: Ok(Default::default()),
                on_pressed: Ok(Default::default()),
                style: Ok(Default::default()),
                tristate: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Checkbox {
        pub fn autofocus<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .autofocus = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autofocus: {}", e)
                });
            self
        }
        pub fn material_tap_target_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesMaterialTapTargetSize>>,
            T::Error: std::fmt::Display,
        {
            self
                .material_tap_target_size = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for material_tap_target_size: {}",
                        e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn on_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_pressed: {}", e)
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesCheckboxStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn tristate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .tristate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tristate: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Checkbox> for super::Checkbox {
        type Error = String;
        fn try_from(value: Checkbox) -> Result<Self, String> {
            Ok(Self {
                autofocus: value.autofocus?,
                material_tap_target_size: value.material_tap_target_size?,
                name: value.name?,
                on_pressed: value.on_pressed?,
                style: value.style?,
                tristate: value.tristate?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Checkbox> for Checkbox {
        fn from(value: super::Checkbox) -> Self {
            Self {
                autofocus: Ok(value.autofocus),
                material_tap_target_size: Ok(value.material_tap_target_size),
                name: Ok(value.name),
                on_pressed: Ok(value.on_pressed),
                style: Ok(value.style),
                tristate: Ok(value.tristate),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Container {
        alignment: Result<Option<super::StylesAlignment>, String>,
        border: Result<Option<super::StylesBorder>, String>,
        child: Result<Option<Box<super::LenraComponent>>, String>,
        constraints: Result<Option<super::StylesBoxConstraints>, String>,
        decoration: Result<Option<super::StylesBoxDecoration>, String>,
        padding: Result<Option<super::StylesPadding>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Container {
        fn default() -> Self {
            Self {
                alignment: Ok(Default::default()),
                border: Ok(Default::default()),
                child: Ok(Default::default()),
                constraints: Ok(Default::default()),
                decoration: Ok(Default::default()),
                padding: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Container {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .alignment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alignment: {}", e)
                });
            self
        }
        pub fn border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .border = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for border: {}", e)
                });
            self
        }
        pub fn child<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::LenraComponent>>>,
            T::Error: std::fmt::Display,
        {
            self
                .child = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for child: {}", e)
                });
            self
        }
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxConstraints>>,
            T::Error: std::fmt::Display,
        {
            self
                .constraints = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for constraints: {}", e)
                });
            self
        }
        pub fn decoration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxDecoration>>,
            T::Error: std::fmt::Display,
        {
            self
                .decoration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decoration: {}", e)
                });
            self
        }
        pub fn padding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesPadding>>,
            T::Error: std::fmt::Display,
        {
            self
                .padding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for padding: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Container> for super::Container {
        type Error = String;
        fn try_from(value: Container) -> Result<Self, String> {
            Ok(Self {
                alignment: value.alignment?,
                border: value.border?,
                child: value.child?,
                constraints: value.constraints?,
                decoration: value.decoration?,
                padding: value.padding?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Container> for Container {
        fn from(value: super::Container) -> Self {
            Self {
                alignment: Ok(value.alignment),
                border: Ok(value.border),
                child: Ok(value.child),
                constraints: Ok(value.constraints),
                decoration: Ok(value.decoration),
                padding: Ok(value.padding),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DropdownButton {
        child: Result<Box<super::LenraComponent>, String>,
        disabled: Result<Option<bool>, String>,
        icon: Result<Option<super::Icon>, String>,
        main_style: Result<Option<super::StylesStyle>, String>,
        size: Result<Option<super::StylesSize>, String>,
        text: Result<String, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for DropdownButton {
        fn default() -> Self {
            Self {
                child: Err("no value supplied for child".to_string()),
                disabled: Ok(Default::default()),
                icon: Ok(Default::default()),
                main_style: Ok(Default::default()),
                size: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DropdownButton {
        pub fn child<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .child = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for child: {}", e)
                });
            self
        }
        pub fn disabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .disabled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for disabled: {}", e)
                });
            self
        }
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
        pub fn main_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .main_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for main_style: {}", e)
                });
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesSize>>,
            T::Error: std::fmt::Display,
        {
            self
                .size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<DropdownButton> for super::DropdownButton {
        type Error = String;
        fn try_from(value: DropdownButton) -> Result<Self, String> {
            Ok(Self {
                child: value.child?,
                disabled: value.disabled?,
                icon: value.icon?,
                main_style: value.main_style?,
                size: value.size?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DropdownButton> for DropdownButton {
        fn from(value: super::DropdownButton) -> Self {
            Self {
                child: Ok(value.child),
                disabled: Ok(value.disabled),
                icon: Ok(value.icon),
                main_style: Ok(value.main_style),
                size: Ok(value.size),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Flex {
        children: Result<Vec<super::LenraComponent>, String>,
        cross_axis_alignment: Result<Option<super::FlexCrossAxisAlignment>, String>,
        direction: Result<Option<super::StylesDirection>, String>,
        fill_parent: Result<Option<bool>, String>,
        horizontal_direction: Result<Option<super::StylesTextDirection>, String>,
        main_axis_alignment: Result<Option<super::FlexMainAxisAlignment>, String>,
        padding: Result<Option<super::StylesPadding>, String>,
        scroll: Result<Option<bool>, String>,
        spacing: Result<Option<f64>, String>,
        text_baseline: Result<Option<super::StylesTextBaseline>, String>,
        type_: Result<serde_json::Value, String>,
        vertical_direction: Result<Option<super::StylesVerticalDirection>, String>,
    }
    impl Default for Flex {
        fn default() -> Self {
            Self {
                children: Err("no value supplied for children".to_string()),
                cross_axis_alignment: Ok(Default::default()),
                direction: Ok(Default::default()),
                fill_parent: Ok(Default::default()),
                horizontal_direction: Ok(Default::default()),
                main_axis_alignment: Ok(Default::default()),
                padding: Ok(Default::default()),
                scroll: Ok(Default::default()),
                spacing: Ok(Default::default()),
                text_baseline: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                vertical_direction: Ok(Default::default()),
            }
        }
    }
    impl Flex {
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .children = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for children: {}", e)
                });
            self
        }
        pub fn cross_axis_alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FlexCrossAxisAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .cross_axis_alignment = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for cross_axis_alignment: {}", e
                    )
                });
            self
        }
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .direction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for direction: {}", e)
                });
            self
        }
        pub fn fill_parent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .fill_parent = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fill_parent: {}", e)
                });
            self
        }
        pub fn horizontal_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .horizontal_direction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for horizontal_direction: {}", e
                    )
                });
            self
        }
        pub fn main_axis_alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FlexMainAxisAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .main_axis_alignment = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for main_axis_alignment: {}", e
                    )
                });
            self
        }
        pub fn padding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesPadding>>,
            T::Error: std::fmt::Display,
        {
            self
                .padding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for padding: {}", e)
                });
            self
        }
        pub fn scroll<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .scroll = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scroll: {}", e)
                });
            self
        }
        pub fn spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .spacing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for spacing: {}", e)
                });
            self
        }
        pub fn text_baseline<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextBaseline>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_baseline = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for text_baseline: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn vertical_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesVerticalDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .vertical_direction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for vertical_direction: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<Flex> for super::Flex {
        type Error = String;
        fn try_from(value: Flex) -> Result<Self, String> {
            Ok(Self {
                children: value.children?,
                cross_axis_alignment: value.cross_axis_alignment?,
                direction: value.direction?,
                fill_parent: value.fill_parent?,
                horizontal_direction: value.horizontal_direction?,
                main_axis_alignment: value.main_axis_alignment?,
                padding: value.padding?,
                scroll: value.scroll?,
                spacing: value.spacing?,
                text_baseline: value.text_baseline?,
                type_: value.type_?,
                vertical_direction: value.vertical_direction?,
            })
        }
    }
    impl From<super::Flex> for Flex {
        fn from(value: super::Flex) -> Self {
            Self {
                children: Ok(value.children),
                cross_axis_alignment: Ok(value.cross_axis_alignment),
                direction: Ok(value.direction),
                fill_parent: Ok(value.fill_parent),
                horizontal_direction: Ok(value.horizontal_direction),
                main_axis_alignment: Ok(value.main_axis_alignment),
                padding: Ok(value.padding),
                scroll: Ok(value.scroll),
                spacing: Ok(value.spacing),
                text_baseline: Ok(value.text_baseline),
                type_: Ok(value.type_),
                vertical_direction: Ok(value.vertical_direction),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Flexible {
        child: Result<Box<super::LenraComponent>, String>,
        fit: Result<Option<super::StylesFlexFit>, String>,
        flex: Result<Option<i64>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Flexible {
        fn default() -> Self {
            Self {
                child: Err("no value supplied for child".to_string()),
                fit: Ok(Default::default()),
                flex: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Flexible {
        pub fn child<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .child = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for child: {}", e)
                });
            self
        }
        pub fn fit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesFlexFit>>,
            T::Error: std::fmt::Display,
        {
            self
                .fit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fit: {}", e));
            self
        }
        pub fn flex<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .flex = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flex: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Flexible> for super::Flexible {
        type Error = String;
        fn try_from(value: Flexible) -> Result<Self, String> {
            Ok(Self {
                child: value.child?,
                fit: value.fit?,
                flex: value.flex?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Flexible> for Flexible {
        fn from(value: super::Flexible) -> Self {
            Self {
                child: Ok(value.child),
                fit: Ok(value.fit),
                flex: Ok(value.flex),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Form {
        child: Result<Box<super::LenraComponent>, String>,
        on_submit: Result<Option<super::Listener>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Form {
        fn default() -> Self {
            Self {
                child: Err("no value supplied for child".to_string()),
                on_submit: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Form {
        pub fn child<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .child = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for child: {}", e)
                });
            self
        }
        pub fn on_submit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_submit = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_submit: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Form> for super::Form {
        type Error = String;
        fn try_from(value: Form) -> Result<Self, String> {
            Ok(Self {
                child: value.child?,
                on_submit: value.on_submit?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Form> for Form {
        fn from(value: super::Form) -> Self {
            Self {
                child: Ok(value.child),
                on_submit: Ok(value.on_submit),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Icon {
        color: Result<Option<super::StylesColor>, String>,
        semantic_label: Result<Option<String>, String>,
        size: Result<Option<f64>, String>,
        style: Result<Option<super::IconDefinitionsIconStyle>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<super::StylesIconName, String>,
    }
    impl Default for Icon {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                semantic_label: Ok(Default::default()),
                size: Ok(Default::default()),
                style: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Icon {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn semantic_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .semantic_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for semantic_label: {}", e)
                });
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IconDefinitionsIconStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StylesIconName>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Icon> for super::Icon {
        type Error = String;
        fn try_from(value: Icon) -> Result<Self, String> {
            Ok(Self {
                color: value.color?,
                semantic_label: value.semantic_label?,
                size: value.size?,
                style: value.style?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Icon> for Icon {
        fn from(value: super::Icon) -> Self {
            Self {
                color: Ok(value.color),
                semantic_label: Ok(value.semantic_label),
                size: Ok(value.size),
                style: Ok(value.style),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Image {
        alignment: Result<Option<super::StylesAlignment>, String>,
        center_slice: Result<Option<super::StylesRect>, String>,
        error_placeholder: Result<Option<Box<super::LenraComponent>>, String>,
        exclude_from_semantics: Result<Option<bool>, String>,
        filter_quality: Result<Option<super::StylesFilterQuality>, String>,
        fit: Result<Option<super::StylesBoxFit>, String>,
        frame_placeholder: Result<Option<Box<super::LenraComponent>>, String>,
        gapless_playback: Result<Option<bool>, String>,
        height: Result<Option<f64>, String>,
        is_anti_alias: Result<Option<bool>, String>,
        loading_placeholder: Result<Option<Box<super::LenraComponent>>, String>,
        repeat: Result<Option<super::StylesImageRepeat>, String>,
        semantic_label: Result<Option<String>, String>,
        src: Result<String, String>,
        type_: Result<serde_json::Value, String>,
        width: Result<Option<f64>, String>,
    }
    impl Default for Image {
        fn default() -> Self {
            Self {
                alignment: Ok(Default::default()),
                center_slice: Ok(Default::default()),
                error_placeholder: Ok(Default::default()),
                exclude_from_semantics: Ok(Default::default()),
                filter_quality: Ok(Default::default()),
                fit: Ok(Default::default()),
                frame_placeholder: Ok(Default::default()),
                gapless_playback: Ok(Default::default()),
                height: Ok(Default::default()),
                is_anti_alias: Ok(Default::default()),
                loading_placeholder: Ok(Default::default()),
                repeat: Ok(Default::default()),
                semantic_label: Ok(Default::default()),
                src: Err("no value supplied for src".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                width: Ok(Default::default()),
            }
        }
    }
    impl Image {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .alignment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alignment: {}", e)
                });
            self
        }
        pub fn center_slice<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesRect>>,
            T::Error: std::fmt::Display,
        {
            self
                .center_slice = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for center_slice: {}", e)
                });
            self
        }
        pub fn error_placeholder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::LenraComponent>>>,
            T::Error: std::fmt::Display,
        {
            self
                .error_placeholder = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for error_placeholder: {}", e
                    )
                });
            self
        }
        pub fn exclude_from_semantics<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .exclude_from_semantics = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for exclude_from_semantics: {}",
                        e
                    )
                });
            self
        }
        pub fn filter_quality<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesFilterQuality>>,
            T::Error: std::fmt::Display,
        {
            self
                .filter_quality = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for filter_quality: {}", e)
                });
            self
        }
        pub fn fit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxFit>>,
            T::Error: std::fmt::Display,
        {
            self
                .fit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fit: {}", e));
            self
        }
        pub fn frame_placeholder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::LenraComponent>>>,
            T::Error: std::fmt::Display,
        {
            self
                .frame_placeholder = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for frame_placeholder: {}", e
                    )
                });
            self
        }
        pub fn gapless_playback<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .gapless_playback = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for gapless_playback: {}", e
                    )
                });
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn is_anti_alias<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .is_anti_alias = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_anti_alias: {}", e)
                });
            self
        }
        pub fn loading_placeholder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::LenraComponent>>>,
            T::Error: std::fmt::Display,
        {
            self
                .loading_placeholder = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for loading_placeholder: {}", e
                    )
                });
            self
        }
        pub fn repeat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesImageRepeat>>,
            T::Error: std::fmt::Display,
        {
            self
                .repeat = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for repeat: {}", e)
                });
            self
        }
        pub fn semantic_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .semantic_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for semantic_label: {}", e)
                });
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for width: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Image> for super::Image {
        type Error = String;
        fn try_from(value: Image) -> Result<Self, String> {
            Ok(Self {
                alignment: value.alignment?,
                center_slice: value.center_slice?,
                error_placeholder: value.error_placeholder?,
                exclude_from_semantics: value.exclude_from_semantics?,
                filter_quality: value.filter_quality?,
                fit: value.fit?,
                frame_placeholder: value.frame_placeholder?,
                gapless_playback: value.gapless_playback?,
                height: value.height?,
                is_anti_alias: value.is_anti_alias?,
                loading_placeholder: value.loading_placeholder?,
                repeat: value.repeat?,
                semantic_label: value.semantic_label?,
                src: value.src?,
                type_: value.type_?,
                width: value.width?,
            })
        }
    }
    impl From<super::Image> for Image {
        fn from(value: super::Image) -> Self {
            Self {
                alignment: Ok(value.alignment),
                center_slice: Ok(value.center_slice),
                error_placeholder: Ok(value.error_placeholder),
                exclude_from_semantics: Ok(value.exclude_from_semantics),
                filter_quality: Ok(value.filter_quality),
                fit: Ok(value.fit),
                frame_placeholder: Ok(value.frame_placeholder),
                gapless_playback: Ok(value.gapless_playback),
                height: Ok(value.height),
                is_anti_alias: Ok(value.is_anti_alias),
                loading_placeholder: Ok(value.loading_placeholder),
                repeat: Ok(value.repeat),
                semantic_label: Ok(value.semantic_label),
                src: Ok(value.src),
                type_: Ok(value.type_),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Listener {
        name: Result<super::ListenerName, String>,
        props: Result<Option<super::DefsProps>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Listener {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                props: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Listener {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListenerName>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn props<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DefsProps>>,
            T::Error: std::fmt::Display,
        {
            self
                .props = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for props: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Listener> for super::Listener {
        type Error = String;
        fn try_from(value: Listener) -> Result<Self, String> {
            Ok(Self {
                name: value.name?,
                props: value.props?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Listener> for Listener {
        fn from(value: super::Listener) -> Self {
            Self {
                name: Ok(value.name),
                props: Ok(value.props),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Menu {
        children: Result<Vec<super::LenraComponent>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Menu {
        fn default() -> Self {
            Self {
                children: Err("no value supplied for children".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Menu {
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .children = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for children: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Menu> for super::Menu {
        type Error = String;
        fn try_from(value: Menu) -> Result<Self, String> {
            Ok(Self {
                children: value.children?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Menu> for Menu {
        fn from(value: super::Menu) -> Self {
            Self {
                children: Ok(value.children),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MenuItem {
        disabled: Result<Option<bool>, String>,
        icon: Result<Option<super::Icon>, String>,
        is_selected: Result<Option<bool>, String>,
        on_pressed: Result<Option<super::Listener>, String>,
        text: Result<String, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for MenuItem {
        fn default() -> Self {
            Self {
                disabled: Ok(Default::default()),
                icon: Ok(Default::default()),
                is_selected: Ok(Default::default()),
                on_pressed: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MenuItem {
        pub fn disabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .disabled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for disabled: {}", e)
                });
            self
        }
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
        pub fn is_selected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .is_selected = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_selected: {}", e)
                });
            self
        }
        pub fn on_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_pressed: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<MenuItem> for super::MenuItem {
        type Error = String;
        fn try_from(value: MenuItem) -> Result<Self, String> {
            Ok(Self {
                disabled: value.disabled?,
                icon: value.icon?,
                is_selected: value.is_selected?,
                on_pressed: value.on_pressed?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::MenuItem> for MenuItem {
        fn from(value: super::MenuItem) -> Self {
            Self {
                disabled: Ok(value.disabled),
                icon: Ok(value.icon),
                is_selected: Ok(value.is_selected),
                on_pressed: Ok(value.on_pressed),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OverlayEntry {
        child: Result<Box<super::LenraComponent>, String>,
        maintain_state: Result<Option<bool>, String>,
        opaque: Result<Option<bool>, String>,
        show_overlay: Result<Option<bool>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for OverlayEntry {
        fn default() -> Self {
            Self {
                child: Err("no value supplied for child".to_string()),
                maintain_state: Ok(Default::default()),
                opaque: Ok(Default::default()),
                show_overlay: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OverlayEntry {
        pub fn child<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .child = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for child: {}", e)
                });
            self
        }
        pub fn maintain_state<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .maintain_state = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for maintain_state: {}", e)
                });
            self
        }
        pub fn opaque<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .opaque = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for opaque: {}", e)
                });
            self
        }
        pub fn show_overlay<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .show_overlay = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for show_overlay: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<OverlayEntry> for super::OverlayEntry {
        type Error = String;
        fn try_from(value: OverlayEntry) -> Result<Self, String> {
            Ok(Self {
                child: value.child?,
                maintain_state: value.maintain_state?,
                opaque: value.opaque?,
                show_overlay: value.show_overlay?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::OverlayEntry> for OverlayEntry {
        fn from(value: super::OverlayEntry) -> Self {
            Self {
                child: Ok(value.child),
                maintain_state: Ok(value.maintain_state),
                opaque: Ok(value.opaque),
                show_overlay: Ok(value.show_overlay),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Radio {
        autofocus: Result<Option<bool>, String>,
        group_value: Result<String, String>,
        material_tap_target_size: Result<
            Option<super::StylesMaterialTapTargetSize>,
            String,
        >,
        name: Result<Option<String>, String>,
        on_pressed: Result<Option<super::Listener>, String>,
        style: Result<Option<super::StylesRadioStyle>, String>,
        toggleable: Result<Option<bool>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<String, String>,
    }
    impl Default for Radio {
        fn default() -> Self {
            Self {
                autofocus: Ok(Default::default()),
                group_value: Err("no value supplied for group_value".to_string()),
                material_tap_target_size: Ok(Default::default()),
                name: Ok(Default::default()),
                on_pressed: Ok(Default::default()),
                style: Ok(Default::default()),
                toggleable: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Radio {
        pub fn autofocus<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .autofocus = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autofocus: {}", e)
                });
            self
        }
        pub fn group_value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .group_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for group_value: {}", e)
                });
            self
        }
        pub fn material_tap_target_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesMaterialTapTargetSize>>,
            T::Error: std::fmt::Display,
        {
            self
                .material_tap_target_size = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for material_tap_target_size: {}",
                        e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn on_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_pressed: {}", e)
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesRadioStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn toggleable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .toggleable = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for toggleable: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Radio> for super::Radio {
        type Error = String;
        fn try_from(value: Radio) -> Result<Self, String> {
            Ok(Self {
                autofocus: value.autofocus?,
                group_value: value.group_value?,
                material_tap_target_size: value.material_tap_target_size?,
                name: value.name?,
                on_pressed: value.on_pressed?,
                style: value.style?,
                toggleable: value.toggleable?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Radio> for Radio {
        fn from(value: super::Radio) -> Self {
            Self {
                autofocus: Ok(value.autofocus),
                group_value: Ok(value.group_value),
                material_tap_target_size: Ok(value.material_tap_target_size),
                name: Ok(value.name),
                on_pressed: Ok(value.on_pressed),
                style: Ok(value.style),
                toggleable: Ok(value.toggleable),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Slider {
        autofocus: Result<Option<bool>, String>,
        divisions: Result<Option<f64>, String>,
        label: Result<Option<String>, String>,
        max: Result<Option<f64>, String>,
        min: Result<Option<f64>, String>,
        name: Result<Option<String>, String>,
        on_change_end: Result<Option<super::Listener>, String>,
        on_change_start: Result<Option<super::Listener>, String>,
        on_changed: Result<Option<super::Listener>, String>,
        style: Result<Option<super::StylesSliderStyle>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<Option<f64>, String>,
    }
    impl Default for Slider {
        fn default() -> Self {
            Self {
                autofocus: Ok(Default::default()),
                divisions: Ok(Default::default()),
                label: Ok(Default::default()),
                max: Ok(Default::default()),
                min: Ok(Default::default()),
                name: Ok(Default::default()),
                on_change_end: Ok(Default::default()),
                on_change_start: Ok(Default::default()),
                on_changed: Ok(Default::default()),
                style: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Ok(Default::default()),
            }
        }
    }
    impl Slider {
        pub fn autofocus<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .autofocus = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autofocus: {}", e)
                });
            self
        }
        pub fn divisions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .divisions = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for divisions: {}", e)
                });
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for label: {}", e)
                });
            self
        }
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn on_change_end<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_change_end = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_change_end: {}", e)
                });
            self
        }
        pub fn on_change_start<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_change_start = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_change_start: {}", e)
                });
            self
        }
        pub fn on_changed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_changed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_changed: {}", e)
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesSliderStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Slider> for super::Slider {
        type Error = String;
        fn try_from(value: Slider) -> Result<Self, String> {
            Ok(Self {
                autofocus: value.autofocus?,
                divisions: value.divisions?,
                label: value.label?,
                max: value.max?,
                min: value.min?,
                name: value.name?,
                on_change_end: value.on_change_end?,
                on_change_start: value.on_change_start?,
                on_changed: value.on_changed?,
                style: value.style?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Slider> for Slider {
        fn from(value: super::Slider) -> Self {
            Self {
                autofocus: Ok(value.autofocus),
                divisions: Ok(value.divisions),
                label: Ok(value.label),
                max: Ok(value.max),
                min: Ok(value.min),
                name: Ok(value.name),
                on_change_end: Ok(value.on_change_end),
                on_change_start: Ok(value.on_change_start),
                on_changed: Ok(value.on_changed),
                style: Ok(value.style),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Stack {
        alignment: Result<Option<super::StylesAlignment>, String>,
        children: Result<Vec<super::LenraComponent>, String>,
        fit: Result<Option<super::StylesStackFit>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Stack {
        fn default() -> Self {
            Self {
                alignment: Ok(Default::default()),
                children: Err("no value supplied for children".to_string()),
                fit: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Stack {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .alignment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alignment: {}", e)
                });
            self
        }
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .children = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for children: {}", e)
                });
            self
        }
        pub fn fit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesStackFit>>,
            T::Error: std::fmt::Display,
        {
            self
                .fit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fit: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Stack> for super::Stack {
        type Error = String;
        fn try_from(value: Stack) -> Result<Self, String> {
            Ok(Self {
                alignment: value.alignment?,
                children: value.children?,
                fit: value.fit?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Stack> for Stack {
        fn from(value: super::Stack) -> Self {
            Self {
                alignment: Ok(value.alignment),
                children: Ok(value.children),
                fit: Ok(value.fit),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StatusSticker {
        status: Result<super::StatusStickerStatus, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for StatusSticker {
        fn default() -> Self {
            Self {
                status: Err("no value supplied for status".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StatusSticker {
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StatusStickerStatus>,
            T::Error: std::fmt::Display,
        {
            self
                .status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StatusSticker> for super::StatusSticker {
        type Error = String;
        fn try_from(value: StatusSticker) -> Result<Self, String> {
            Ok(Self {
                status: value.status?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::StatusSticker> for StatusSticker {
        fn from(value: super::StatusSticker) -> Self {
            Self {
                status: Ok(value.status),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBorder {
        bottom: Result<Option<super::StylesBorderSide>, String>,
        left: Result<Option<super::StylesBorderSide>, String>,
        right: Result<Option<super::StylesBorderSide>, String>,
        top: Result<Option<super::StylesBorderSide>, String>,
    }
    impl Default for StylesBorder {
        fn default() -> Self {
            Self {
                bottom: Ok(Default::default()),
                left: Ok(Default::default()),
                right: Ok(Default::default()),
                top: Ok(Default::default()),
            }
        }
    }
    impl StylesBorder {
        pub fn bottom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderSide>>,
            T::Error: std::fmt::Display,
        {
            self
                .bottom = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bottom: {}", e)
                });
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderSide>>,
            T::Error: std::fmt::Display,
        {
            self
                .left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderSide>>,
            T::Error: std::fmt::Display,
        {
            self
                .right = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for right: {}", e)
                });
            self
        }
        pub fn top<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderSide>>,
            T::Error: std::fmt::Display,
        {
            self
                .top = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StylesBorder> for super::StylesBorder {
        type Error = String;
        fn try_from(value: StylesBorder) -> Result<Self, String> {
            Ok(Self {
                bottom: value.bottom?,
                left: value.left?,
                right: value.right?,
                top: value.top?,
            })
        }
    }
    impl From<super::StylesBorder> for StylesBorder {
        fn from(value: super::StylesBorder) -> Self {
            Self {
                bottom: Ok(value.bottom),
                left: Ok(value.left),
                right: Ok(value.right),
                top: Ok(value.top),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBorderRadius {
        bottom_left: Result<
            Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            String,
        >,
        bottom_right: Result<
            Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            String,
        >,
        top_left: Result<Option<super::StylesBorderRadiusDefinitionsRadiusType>, String>,
        top_right: Result<
            Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            String,
        >,
    }
    impl Default for StylesBorderRadius {
        fn default() -> Self {
            Self {
                bottom_left: Ok(Default::default()),
                bottom_right: Ok(Default::default()),
                top_left: Ok(Default::default()),
                top_right: Ok(Default::default()),
            }
        }
    }
    impl StylesBorderRadius {
        pub fn bottom_left<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .bottom_left = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bottom_left: {}", e)
                });
            self
        }
        pub fn bottom_right<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .bottom_right = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bottom_right: {}", e)
                });
            self
        }
        pub fn top_left<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .top_left = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for top_left: {}", e)
                });
            self
        }
        pub fn top_right<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::StylesBorderRadiusDefinitionsRadiusType>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .top_right = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for top_right: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesBorderRadius> for super::StylesBorderRadius {
        type Error = String;
        fn try_from(value: StylesBorderRadius) -> Result<Self, String> {
            Ok(Self {
                bottom_left: value.bottom_left?,
                bottom_right: value.bottom_right?,
                top_left: value.top_left?,
                top_right: value.top_right?,
            })
        }
    }
    impl From<super::StylesBorderRadius> for StylesBorderRadius {
        fn from(value: super::StylesBorderRadius) -> Self {
            Self {
                bottom_left: Ok(value.bottom_left),
                bottom_right: Ok(value.bottom_right),
                top_left: Ok(value.top_left),
                top_right: Ok(value.top_right),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBorderRadiusDefinitionsRadiusType {
        x: Result<Option<f64>, String>,
        y: Result<Option<f64>, String>,
    }
    impl Default for StylesBorderRadiusDefinitionsRadiusType {
        fn default() -> Self {
            Self {
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl StylesBorderRadiusDefinitionsRadiusType {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StylesBorderRadiusDefinitionsRadiusType>
    for super::StylesBorderRadiusDefinitionsRadiusType {
        type Error = String;
        fn try_from(
            value: StylesBorderRadiusDefinitionsRadiusType,
        ) -> Result<Self, String> {
            Ok(Self { x: value.x?, y: value.y? })
        }
    }
    impl From<super::StylesBorderRadiusDefinitionsRadiusType>
    for StylesBorderRadiusDefinitionsRadiusType {
        fn from(value: super::StylesBorderRadiusDefinitionsRadiusType) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBorderSide {
        color: Result<Option<super::StylesColor>, String>,
        width: Result<Option<f64>, String>,
    }
    impl Default for StylesBorderSide {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl StylesBorderSide {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for width: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesBorderSide> for super::StylesBorderSide {
        type Error = String;
        fn try_from(value: StylesBorderSide) -> Result<Self, String> {
            Ok(Self {
                color: value.color?,
                width: value.width?,
            })
        }
    }
    impl From<super::StylesBorderSide> for StylesBorderSide {
        fn from(value: super::StylesBorderSide) -> Self {
            Self {
                color: Ok(value.color),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBoxConstraints {
        max_height: Result<Option<f64>, String>,
        max_width: Result<Option<f64>, String>,
        min_height: Result<Option<f64>, String>,
        min_width: Result<Option<f64>, String>,
    }
    impl Default for StylesBoxConstraints {
        fn default() -> Self {
            Self {
                max_height: Ok(Default::default()),
                max_width: Ok(Default::default()),
                min_height: Ok(Default::default()),
                min_width: Ok(Default::default()),
            }
        }
    }
    impl StylesBoxConstraints {
        pub fn max_height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .max_height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for max_height: {}", e)
                });
            self
        }
        pub fn max_width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .max_width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for max_width: {}", e)
                });
            self
        }
        pub fn min_height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .min_height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for min_height: {}", e)
                });
            self
        }
        pub fn min_width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .min_width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for min_width: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesBoxConstraints> for super::StylesBoxConstraints {
        type Error = String;
        fn try_from(value: StylesBoxConstraints) -> Result<Self, String> {
            Ok(Self {
                max_height: value.max_height?,
                max_width: value.max_width?,
                min_height: value.min_height?,
                min_width: value.min_width?,
            })
        }
    }
    impl From<super::StylesBoxConstraints> for StylesBoxConstraints {
        fn from(value: super::StylesBoxConstraints) -> Self {
            Self {
                max_height: Ok(value.max_height),
                max_width: Ok(value.max_width),
                min_height: Ok(value.min_height),
                min_width: Ok(value.min_width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBoxDecoration {
        border_radius: Result<Option<super::StylesBorderRadius>, String>,
        box_shadow: Result<Option<super::StylesBoxShadow>, String>,
        color: Result<Option<super::StylesColor>, String>,
        shape: Result<Option<super::StylesBoxShape>, String>,
    }
    impl Default for StylesBoxDecoration {
        fn default() -> Self {
            Self {
                border_radius: Ok(Default::default()),
                box_shadow: Ok(Default::default()),
                color: Ok(Default::default()),
                shape: Ok(Default::default()),
            }
        }
    }
    impl StylesBoxDecoration {
        pub fn border_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderRadius>>,
            T::Error: std::fmt::Display,
        {
            self
                .border_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for border_radius: {}", e)
                });
            self
        }
        pub fn box_shadow<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxShadow>>,
            T::Error: std::fmt::Display,
        {
            self
                .box_shadow = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for box_shadow: {}", e)
                });
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn shape<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxShape>>,
            T::Error: std::fmt::Display,
        {
            self
                .shape = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for shape: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesBoxDecoration> for super::StylesBoxDecoration {
        type Error = String;
        fn try_from(value: StylesBoxDecoration) -> Result<Self, String> {
            Ok(Self {
                border_radius: value.border_radius?,
                box_shadow: value.box_shadow?,
                color: value.color?,
                shape: value.shape?,
            })
        }
    }
    impl From<super::StylesBoxDecoration> for StylesBoxDecoration {
        fn from(value: super::StylesBoxDecoration) -> Self {
            Self {
                border_radius: Ok(value.border_radius),
                box_shadow: Ok(value.box_shadow),
                color: Ok(value.color),
                shape: Ok(value.shape),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesBoxShadow {
        blur_radius: Result<Option<f64>, String>,
        color: Result<Option<super::StylesColor>, String>,
        offset: Result<Option<super::StylesOffset>, String>,
        spread_radius: Result<Option<f64>, String>,
    }
    impl Default for StylesBoxShadow {
        fn default() -> Self {
            Self {
                blur_radius: Ok(Default::default()),
                color: Ok(Default::default()),
                offset: Ok(Default::default()),
                spread_radius: Ok(Default::default()),
            }
        }
    }
    impl StylesBoxShadow {
        pub fn blur_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .blur_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for blur_radius: {}", e)
                });
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn offset<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesOffset>>,
            T::Error: std::fmt::Display,
        {
            self
                .offset = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for offset: {}", e)
                });
            self
        }
        pub fn spread_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .spread_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for spread_radius: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesBoxShadow> for super::StylesBoxShadow {
        type Error = String;
        fn try_from(value: StylesBoxShadow) -> Result<Self, String> {
            Ok(Self {
                blur_radius: value.blur_radius?,
                color: value.color?,
                offset: value.offset?,
                spread_radius: value.spread_radius?,
            })
        }
    }
    impl From<super::StylesBoxShadow> for StylesBoxShadow {
        fn from(value: super::StylesBoxShadow) -> Self {
            Self {
                blur_radius: Ok(value.blur_radius),
                color: Ok(value.color),
                offset: Ok(value.offset),
                spread_radius: Ok(value.spread_radius),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesCarouselOptions {
        aspect_ratio: Result<Option<f64>, String>,
        auto_play: Result<Option<bool>, String>,
        auto_play_animation_duration: Result<Option<super::StylesDuration>, String>,
        auto_play_interval: Result<Option<super::StylesDuration>, String>,
        enable_infinite_scroll: Result<Option<bool>, String>,
        enlarge_center_page: Result<Option<bool>, String>,
        enlarge_strategy: Result<
            Option<super::StylesCarouselOptionsEnlargeStrategy>,
            String,
        >,
        height: Result<Option<f64>, String>,
        initial_page: Result<Option<i64>, String>,
        pause_auto_play_on_touch: Result<Option<bool>, String>,
        reverse: Result<Option<bool>, String>,
        scroll_direction: Result<Option<super::StylesDirection>, String>,
        viewport_fraction: Result<Option<f64>, String>,
    }
    impl Default for StylesCarouselOptions {
        fn default() -> Self {
            Self {
                aspect_ratio: Ok(Default::default()),
                auto_play: Ok(Default::default()),
                auto_play_animation_duration: Ok(Default::default()),
                auto_play_interval: Ok(Default::default()),
                enable_infinite_scroll: Ok(Default::default()),
                enlarge_center_page: Ok(Default::default()),
                enlarge_strategy: Ok(Default::default()),
                height: Ok(Default::default()),
                initial_page: Ok(Default::default()),
                pause_auto_play_on_touch: Ok(Default::default()),
                reverse: Ok(Default::default()),
                scroll_direction: Ok(Default::default()),
                viewport_fraction: Ok(Default::default()),
            }
        }
    }
    impl StylesCarouselOptions {
        pub fn aspect_ratio<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .aspect_ratio = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for aspect_ratio: {}", e)
                });
            self
        }
        pub fn auto_play<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .auto_play = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for auto_play: {}", e)
                });
            self
        }
        pub fn auto_play_animation_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesDuration>>,
            T::Error: std::fmt::Display,
        {
            self
                .auto_play_animation_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for auto_play_animation_duration: {}",
                        e
                    )
                });
            self
        }
        pub fn auto_play_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesDuration>>,
            T::Error: std::fmt::Display,
        {
            self
                .auto_play_interval = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for auto_play_interval: {}", e
                    )
                });
            self
        }
        pub fn enable_infinite_scroll<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .enable_infinite_scroll = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for enable_infinite_scroll: {}",
                        e
                    )
                });
            self
        }
        pub fn enlarge_center_page<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .enlarge_center_page = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for enlarge_center_page: {}", e
                    )
                });
            self
        }
        pub fn enlarge_strategy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::StylesCarouselOptionsEnlargeStrategy>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .enlarge_strategy = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for enlarge_strategy: {}", e
                    )
                });
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn initial_page<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .initial_page = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for initial_page: {}", e)
                });
            self
        }
        pub fn pause_auto_play_on_touch<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .pause_auto_play_on_touch = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for pause_auto_play_on_touch: {}",
                        e
                    )
                });
            self
        }
        pub fn reverse<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .reverse = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reverse: {}", e)
                });
            self
        }
        pub fn scroll_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .scroll_direction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for scroll_direction: {}", e
                    )
                });
            self
        }
        pub fn viewport_fraction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .viewport_fraction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for viewport_fraction: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesCarouselOptions> for super::StylesCarouselOptions {
        type Error = String;
        fn try_from(value: StylesCarouselOptions) -> Result<Self, String> {
            Ok(Self {
                aspect_ratio: value.aspect_ratio?,
                auto_play: value.auto_play?,
                auto_play_animation_duration: value.auto_play_animation_duration?,
                auto_play_interval: value.auto_play_interval?,
                enable_infinite_scroll: value.enable_infinite_scroll?,
                enlarge_center_page: value.enlarge_center_page?,
                enlarge_strategy: value.enlarge_strategy?,
                height: value.height?,
                initial_page: value.initial_page?,
                pause_auto_play_on_touch: value.pause_auto_play_on_touch?,
                reverse: value.reverse?,
                scroll_direction: value.scroll_direction?,
                viewport_fraction: value.viewport_fraction?,
            })
        }
    }
    impl From<super::StylesCarouselOptions> for StylesCarouselOptions {
        fn from(value: super::StylesCarouselOptions) -> Self {
            Self {
                aspect_ratio: Ok(value.aspect_ratio),
                auto_play: Ok(value.auto_play),
                auto_play_animation_duration: Ok(value.auto_play_animation_duration),
                auto_play_interval: Ok(value.auto_play_interval),
                enable_infinite_scroll: Ok(value.enable_infinite_scroll),
                enlarge_center_page: Ok(value.enlarge_center_page),
                enlarge_strategy: Ok(value.enlarge_strategy),
                height: Ok(value.height),
                initial_page: Ok(value.initial_page),
                pause_auto_play_on_touch: Ok(value.pause_auto_play_on_touch),
                reverse: Ok(value.reverse),
                scroll_direction: Ok(value.scroll_direction),
                viewport_fraction: Ok(value.viewport_fraction),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesCheckboxStyle {
        active_color: Result<Option<super::StylesColor>, String>,
        check_color: Result<Option<super::StylesColor>, String>,
        focus_color: Result<Option<super::StylesColor>, String>,
        hover_color: Result<Option<super::StylesColor>, String>,
        shape: Result<Option<super::StylesOutlinedBorder>, String>,
        side: Result<Option<super::StylesBorderSide>, String>,
        splash_radius: Result<Option<f64>, String>,
        visual_density: Result<Option<super::StylesVisualDensity>, String>,
    }
    impl Default for StylesCheckboxStyle {
        fn default() -> Self {
            Self {
                active_color: Ok(Default::default()),
                check_color: Ok(Default::default()),
                focus_color: Ok(Default::default()),
                hover_color: Ok(Default::default()),
                shape: Ok(Default::default()),
                side: Ok(Default::default()),
                splash_radius: Ok(Default::default()),
                visual_density: Ok(Default::default()),
            }
        }
    }
    impl StylesCheckboxStyle {
        pub fn active_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .active_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for active_color: {}", e)
                });
            self
        }
        pub fn check_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .check_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for check_color: {}", e)
                });
            self
        }
        pub fn focus_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .focus_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for focus_color: {}", e)
                });
            self
        }
        pub fn hover_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .hover_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hover_color: {}", e)
                });
            self
        }
        pub fn shape<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesOutlinedBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .shape = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for shape: {}", e)
                });
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderSide>>,
            T::Error: std::fmt::Display,
        {
            self
                .side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {}", e));
            self
        }
        pub fn splash_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .splash_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for splash_radius: {}", e)
                });
            self
        }
        pub fn visual_density<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesVisualDensity>>,
            T::Error: std::fmt::Display,
        {
            self
                .visual_density = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for visual_density: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesCheckboxStyle> for super::StylesCheckboxStyle {
        type Error = String;
        fn try_from(value: StylesCheckboxStyle) -> Result<Self, String> {
            Ok(Self {
                active_color: value.active_color?,
                check_color: value.check_color?,
                focus_color: value.focus_color?,
                hover_color: value.hover_color?,
                shape: value.shape?,
                side: value.side?,
                splash_radius: value.splash_radius?,
                visual_density: value.visual_density?,
            })
        }
    }
    impl From<super::StylesCheckboxStyle> for StylesCheckboxStyle {
        fn from(value: super::StylesCheckboxStyle) -> Self {
            Self {
                active_color: Ok(value.active_color),
                check_color: Ok(value.check_color),
                focus_color: Ok(value.focus_color),
                hover_color: Ok(value.hover_color),
                shape: Ok(value.shape),
                side: Ok(value.side),
                splash_radius: Ok(value.splash_radius),
                visual_density: Ok(value.visual_density),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesDuration {
        days: Result<Option<i64>, String>,
        hours: Result<Option<i64>, String>,
        microseconds: Result<Option<i64>, String>,
        milliseconds: Result<Option<i64>, String>,
        minutes: Result<Option<i64>, String>,
        seconds: Result<Option<i64>, String>,
    }
    impl Default for StylesDuration {
        fn default() -> Self {
            Self {
                days: Ok(Default::default()),
                hours: Ok(Default::default()),
                microseconds: Ok(Default::default()),
                milliseconds: Ok(Default::default()),
                minutes: Ok(Default::default()),
                seconds: Ok(Default::default()),
            }
        }
    }
    impl StylesDuration {
        pub fn days<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for days: {}", e));
            self
        }
        pub fn hours<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .hours = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hours: {}", e)
                });
            self
        }
        pub fn microseconds<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .microseconds = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for microseconds: {}", e)
                });
            self
        }
        pub fn milliseconds<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .milliseconds = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for milliseconds: {}", e)
                });
            self
        }
        pub fn minutes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .minutes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for minutes: {}", e)
                });
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .seconds = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for seconds: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesDuration> for super::StylesDuration {
        type Error = String;
        fn try_from(value: StylesDuration) -> Result<Self, String> {
            Ok(Self {
                days: value.days?,
                hours: value.hours?,
                microseconds: value.microseconds?,
                milliseconds: value.milliseconds?,
                minutes: value.minutes?,
                seconds: value.seconds?,
            })
        }
    }
    impl From<super::StylesDuration> for StylesDuration {
        fn from(value: super::StylesDuration) -> Self {
            Self {
                days: Ok(value.days),
                hours: Ok(value.hours),
                microseconds: Ok(value.microseconds),
                milliseconds: Ok(value.milliseconds),
                minutes: Ok(value.minutes),
                seconds: Ok(value.seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesInputBorder {
        border_radius: Result<Option<super::StylesBorderRadius>, String>,
        border_side: Result<super::StylesBorderSide, String>,
        type_: Result<Option<super::StylesInputBorderType>, String>,
    }
    impl Default for StylesInputBorder {
        fn default() -> Self {
            Self {
                border_radius: Ok(Default::default()),
                border_side: Err("no value supplied for border_side".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl StylesInputBorder {
        pub fn border_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderRadius>>,
            T::Error: std::fmt::Display,
        {
            self
                .border_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for border_radius: {}", e)
                });
            self
        }
        pub fn border_side<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StylesBorderSide>,
            T::Error: std::fmt::Display,
        {
            self
                .border_side = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for border_side: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorderType>>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesInputBorder> for super::StylesInputBorder {
        type Error = String;
        fn try_from(value: StylesInputBorder) -> Result<Self, String> {
            Ok(Self {
                border_radius: value.border_radius?,
                border_side: value.border_side?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::StylesInputBorder> for StylesInputBorder {
        fn from(value: super::StylesInputBorder) -> Self {
            Self {
                border_radius: Ok(value.border_radius),
                border_side: Ok(value.border_side),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesInputDecoration {
        align_label_with_hint: Result<Option<bool>, String>,
        border: Result<Option<super::StylesInputBorder>, String>,
        constraints: Result<Option<super::StylesBoxConstraints>, String>,
        content_padding: Result<Option<super::StylesPadding>, String>,
        counter: Result<serde_json::Map<String, serde_json::Value>, String>,
        counter_style: Result<Option<super::StylesTextStyle>, String>,
        counter_text: Result<Option<String>, String>,
        disabled_border: Result<Option<super::StylesInputBorder>, String>,
        enabled: Result<Option<bool>, String>,
        enabled_border: Result<Option<super::StylesInputBorder>, String>,
        error_border: Result<Option<super::StylesInputBorder>, String>,
        error_max_lines: Result<Option<i64>, String>,
        error_style: Result<Option<super::StylesTextStyle>, String>,
        error_text: Result<Option<String>, String>,
        fill_color: Result<Option<super::StylesColor>, String>,
        filled: Result<Option<bool>, String>,
        floating_label_behavior: Result<
            Option<super::StylesFloatingLabelBehavior>,
            String,
        >,
        floating_label_style: Result<Option<super::StylesTextStyle>, String>,
        focus_color: Result<Option<super::StylesColor>, String>,
        focused_border: Result<Option<super::StylesInputBorder>, String>,
        focused_error_border: Result<Option<super::StylesInputBorder>, String>,
        helper_max_lines: Result<Option<i64>, String>,
        helper_style: Result<Option<super::StylesTextStyle>, String>,
        helper_text: Result<Option<String>, String>,
        hint_max_lines: Result<Option<i64>, String>,
        hint_style: Result<Option<super::StylesTextStyle>, String>,
        hint_text: Result<Option<String>, String>,
        hint_text_direction: Result<Option<super::StylesTextDirection>, String>,
        hover_color: Result<Option<super::StylesColor>, String>,
        icon: Result<Option<super::Icon>, String>,
        icon_color: Result<Option<super::StylesColor>, String>,
        is_collapsed: Result<Option<bool>, String>,
        is_dense: Result<Option<bool>, String>,
        label: Result<Option<Box<super::LenraComponent>>, String>,
        label_style: Result<Option<super::StylesTextStyle>, String>,
        label_text: Result<Option<String>, String>,
        prefix: Result<Option<Box<super::LenraComponent>>, String>,
        prefix_icon: Result<Option<super::Icon>, String>,
        prefix_icon_color: Result<Option<super::StylesColor>, String>,
        prefix_icon_constraints: Result<Option<super::StylesBoxConstraints>, String>,
        prefix_style: Result<Option<super::StylesTextStyle>, String>,
        prefix_text: Result<Option<String>, String>,
        semantic_counter_text: Result<Option<String>, String>,
        suffix: Result<serde_json::Map<String, serde_json::Value>, String>,
        suffix_icon: Result<Option<super::Icon>, String>,
        suffix_icon_color: Result<Option<super::StylesColor>, String>,
        suffix_icon_constraints: Result<Option<super::StylesBoxConstraints>, String>,
        suffix_style: Result<Option<super::StylesTextStyle>, String>,
        suffix_text: Result<Option<String>, String>,
    }
    impl Default for StylesInputDecoration {
        fn default() -> Self {
            Self {
                align_label_with_hint: Ok(Default::default()),
                border: Ok(Default::default()),
                constraints: Ok(Default::default()),
                content_padding: Ok(Default::default()),
                counter: Ok(Default::default()),
                counter_style: Ok(Default::default()),
                counter_text: Ok(Default::default()),
                disabled_border: Ok(Default::default()),
                enabled: Ok(Default::default()),
                enabled_border: Ok(Default::default()),
                error_border: Ok(Default::default()),
                error_max_lines: Ok(Default::default()),
                error_style: Ok(Default::default()),
                error_text: Ok(Default::default()),
                fill_color: Ok(Default::default()),
                filled: Ok(Default::default()),
                floating_label_behavior: Ok(Default::default()),
                floating_label_style: Ok(Default::default()),
                focus_color: Ok(Default::default()),
                focused_border: Ok(Default::default()),
                focused_error_border: Ok(Default::default()),
                helper_max_lines: Ok(Default::default()),
                helper_style: Ok(Default::default()),
                helper_text: Ok(Default::default()),
                hint_max_lines: Ok(Default::default()),
                hint_style: Ok(Default::default()),
                hint_text: Ok(Default::default()),
                hint_text_direction: Ok(Default::default()),
                hover_color: Ok(Default::default()),
                icon: Ok(Default::default()),
                icon_color: Ok(Default::default()),
                is_collapsed: Ok(Default::default()),
                is_dense: Ok(Default::default()),
                label: Ok(Default::default()),
                label_style: Ok(Default::default()),
                label_text: Ok(Default::default()),
                prefix: Ok(Default::default()),
                prefix_icon: Ok(Default::default()),
                prefix_icon_color: Ok(Default::default()),
                prefix_icon_constraints: Ok(Default::default()),
                prefix_style: Ok(Default::default()),
                prefix_text: Ok(Default::default()),
                semantic_counter_text: Ok(Default::default()),
                suffix: Ok(Default::default()),
                suffix_icon: Ok(Default::default()),
                suffix_icon_color: Ok(Default::default()),
                suffix_icon_constraints: Ok(Default::default()),
                suffix_style: Ok(Default::default()),
                suffix_text: Ok(Default::default()),
            }
        }
    }
    impl StylesInputDecoration {
        pub fn align_label_with_hint<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .align_label_with_hint = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for align_label_with_hint: {}",
                        e
                    )
                });
            self
        }
        pub fn border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .border = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for border: {}", e)
                });
            self
        }
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxConstraints>>,
            T::Error: std::fmt::Display,
        {
            self
                .constraints = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for constraints: {}", e)
                });
            self
        }
        pub fn content_padding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesPadding>>,
            T::Error: std::fmt::Display,
        {
            self
                .content_padding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content_padding: {}", e)
                });
            self
        }
        pub fn counter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self
                .counter = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for counter: {}", e)
                });
            self
        }
        pub fn counter_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .counter_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for counter_style: {}", e)
                });
            self
        }
        pub fn counter_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .counter_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for counter_text: {}", e)
                });
            self
        }
        pub fn disabled_border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .disabled_border = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for disabled_border: {}", e)
                });
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .enabled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for enabled: {}", e)
                });
            self
        }
        pub fn enabled_border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .enabled_border = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for enabled_border: {}", e)
                });
            self
        }
        pub fn error_border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .error_border = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error_border: {}", e)
                });
            self
        }
        pub fn error_max_lines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .error_max_lines = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error_max_lines: {}", e)
                });
            self
        }
        pub fn error_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .error_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error_style: {}", e)
                });
            self
        }
        pub fn error_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .error_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error_text: {}", e)
                });
            self
        }
        pub fn fill_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .fill_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fill_color: {}", e)
                });
            self
        }
        pub fn filled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .filled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for filled: {}", e)
                });
            self
        }
        pub fn floating_label_behavior<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesFloatingLabelBehavior>>,
            T::Error: std::fmt::Display,
        {
            self
                .floating_label_behavior = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for floating_label_behavior: {}",
                        e
                    )
                });
            self
        }
        pub fn floating_label_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .floating_label_style = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for floating_label_style: {}", e
                    )
                });
            self
        }
        pub fn focus_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .focus_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for focus_color: {}", e)
                });
            self
        }
        pub fn focused_border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .focused_border = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for focused_border: {}", e)
                });
            self
        }
        pub fn focused_error_border<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputBorder>>,
            T::Error: std::fmt::Display,
        {
            self
                .focused_error_border = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for focused_error_border: {}", e
                    )
                });
            self
        }
        pub fn helper_max_lines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .helper_max_lines = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for helper_max_lines: {}", e
                    )
                });
            self
        }
        pub fn helper_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .helper_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for helper_style: {}", e)
                });
            self
        }
        pub fn helper_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .helper_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for helper_text: {}", e)
                });
            self
        }
        pub fn hint_max_lines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .hint_max_lines = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hint_max_lines: {}", e)
                });
            self
        }
        pub fn hint_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .hint_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hint_style: {}", e)
                });
            self
        }
        pub fn hint_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .hint_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hint_text: {}", e)
                });
            self
        }
        pub fn hint_text_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .hint_text_direction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for hint_text_direction: {}", e
                    )
                });
            self
        }
        pub fn hover_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .hover_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hover_color: {}", e)
                });
            self
        }
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
        pub fn icon_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .icon_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for icon_color: {}", e)
                });
            self
        }
        pub fn is_collapsed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .is_collapsed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_collapsed: {}", e)
                });
            self
        }
        pub fn is_dense<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .is_dense = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_dense: {}", e)
                });
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::LenraComponent>>>,
            T::Error: std::fmt::Display,
        {
            self
                .label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for label: {}", e)
                });
            self
        }
        pub fn label_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .label_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for label_style: {}", e)
                });
            self
        }
        pub fn label_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .label_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for label_text: {}", e)
                });
            self
        }
        pub fn prefix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::LenraComponent>>>,
            T::Error: std::fmt::Display,
        {
            self
                .prefix = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prefix: {}", e)
                });
            self
        }
        pub fn prefix_icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .prefix_icon = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prefix_icon: {}", e)
                });
            self
        }
        pub fn prefix_icon_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .prefix_icon_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for prefix_icon_color: {}", e
                    )
                });
            self
        }
        pub fn prefix_icon_constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxConstraints>>,
            T::Error: std::fmt::Display,
        {
            self
                .prefix_icon_constraints = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for prefix_icon_constraints: {}",
                        e
                    )
                });
            self
        }
        pub fn prefix_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .prefix_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prefix_style: {}", e)
                });
            self
        }
        pub fn prefix_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .prefix_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prefix_text: {}", e)
                });
            self
        }
        pub fn semantic_counter_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .semantic_counter_text = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for semantic_counter_text: {}",
                        e
                    )
                });
            self
        }
        pub fn suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self
                .suffix = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for suffix: {}", e)
                });
            self
        }
        pub fn suffix_icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Icon>>,
            T::Error: std::fmt::Display,
        {
            self
                .suffix_icon = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for suffix_icon: {}", e)
                });
            self
        }
        pub fn suffix_icon_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .suffix_icon_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for suffix_icon_color: {}", e
                    )
                });
            self
        }
        pub fn suffix_icon_constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxConstraints>>,
            T::Error: std::fmt::Display,
        {
            self
                .suffix_icon_constraints = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for suffix_icon_constraints: {}",
                        e
                    )
                });
            self
        }
        pub fn suffix_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .suffix_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for suffix_style: {}", e)
                });
            self
        }
        pub fn suffix_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .suffix_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for suffix_text: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesInputDecoration> for super::StylesInputDecoration {
        type Error = String;
        fn try_from(value: StylesInputDecoration) -> Result<Self, String> {
            Ok(Self {
                align_label_with_hint: value.align_label_with_hint?,
                border: value.border?,
                constraints: value.constraints?,
                content_padding: value.content_padding?,
                counter: value.counter?,
                counter_style: value.counter_style?,
                counter_text: value.counter_text?,
                disabled_border: value.disabled_border?,
                enabled: value.enabled?,
                enabled_border: value.enabled_border?,
                error_border: value.error_border?,
                error_max_lines: value.error_max_lines?,
                error_style: value.error_style?,
                error_text: value.error_text?,
                fill_color: value.fill_color?,
                filled: value.filled?,
                floating_label_behavior: value.floating_label_behavior?,
                floating_label_style: value.floating_label_style?,
                focus_color: value.focus_color?,
                focused_border: value.focused_border?,
                focused_error_border: value.focused_error_border?,
                helper_max_lines: value.helper_max_lines?,
                helper_style: value.helper_style?,
                helper_text: value.helper_text?,
                hint_max_lines: value.hint_max_lines?,
                hint_style: value.hint_style?,
                hint_text: value.hint_text?,
                hint_text_direction: value.hint_text_direction?,
                hover_color: value.hover_color?,
                icon: value.icon?,
                icon_color: value.icon_color?,
                is_collapsed: value.is_collapsed?,
                is_dense: value.is_dense?,
                label: value.label?,
                label_style: value.label_style?,
                label_text: value.label_text?,
                prefix: value.prefix?,
                prefix_icon: value.prefix_icon?,
                prefix_icon_color: value.prefix_icon_color?,
                prefix_icon_constraints: value.prefix_icon_constraints?,
                prefix_style: value.prefix_style?,
                prefix_text: value.prefix_text?,
                semantic_counter_text: value.semantic_counter_text?,
                suffix: value.suffix?,
                suffix_icon: value.suffix_icon?,
                suffix_icon_color: value.suffix_icon_color?,
                suffix_icon_constraints: value.suffix_icon_constraints?,
                suffix_style: value.suffix_style?,
                suffix_text: value.suffix_text?,
            })
        }
    }
    impl From<super::StylesInputDecoration> for StylesInputDecoration {
        fn from(value: super::StylesInputDecoration) -> Self {
            Self {
                align_label_with_hint: Ok(value.align_label_with_hint),
                border: Ok(value.border),
                constraints: Ok(value.constraints),
                content_padding: Ok(value.content_padding),
                counter: Ok(value.counter),
                counter_style: Ok(value.counter_style),
                counter_text: Ok(value.counter_text),
                disabled_border: Ok(value.disabled_border),
                enabled: Ok(value.enabled),
                enabled_border: Ok(value.enabled_border),
                error_border: Ok(value.error_border),
                error_max_lines: Ok(value.error_max_lines),
                error_style: Ok(value.error_style),
                error_text: Ok(value.error_text),
                fill_color: Ok(value.fill_color),
                filled: Ok(value.filled),
                floating_label_behavior: Ok(value.floating_label_behavior),
                floating_label_style: Ok(value.floating_label_style),
                focus_color: Ok(value.focus_color),
                focused_border: Ok(value.focused_border),
                focused_error_border: Ok(value.focused_error_border),
                helper_max_lines: Ok(value.helper_max_lines),
                helper_style: Ok(value.helper_style),
                helper_text: Ok(value.helper_text),
                hint_max_lines: Ok(value.hint_max_lines),
                hint_style: Ok(value.hint_style),
                hint_text: Ok(value.hint_text),
                hint_text_direction: Ok(value.hint_text_direction),
                hover_color: Ok(value.hover_color),
                icon: Ok(value.icon),
                icon_color: Ok(value.icon_color),
                is_collapsed: Ok(value.is_collapsed),
                is_dense: Ok(value.is_dense),
                label: Ok(value.label),
                label_style: Ok(value.label_style),
                label_text: Ok(value.label_text),
                prefix: Ok(value.prefix),
                prefix_icon: Ok(value.prefix_icon),
                prefix_icon_color: Ok(value.prefix_icon_color),
                prefix_icon_constraints: Ok(value.prefix_icon_constraints),
                prefix_style: Ok(value.prefix_style),
                prefix_text: Ok(value.prefix_text),
                semantic_counter_text: Ok(value.semantic_counter_text),
                suffix: Ok(value.suffix),
                suffix_icon: Ok(value.suffix_icon),
                suffix_icon_color: Ok(value.suffix_icon_color),
                suffix_icon_constraints: Ok(value.suffix_icon_constraints),
                suffix_style: Ok(value.suffix_style),
                suffix_text: Ok(value.suffix_text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesLocale {
        country_code: Result<Option<String>, String>,
        language_code: Result<Option<String>, String>,
        script_code: Result<Option<String>, String>,
    }
    impl Default for StylesLocale {
        fn default() -> Self {
            Self {
                country_code: Ok(Default::default()),
                language_code: Ok(Default::default()),
                script_code: Ok(Default::default()),
            }
        }
    }
    impl StylesLocale {
        pub fn country_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .country_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country_code: {}", e)
                });
            self
        }
        pub fn language_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .language_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language_code: {}", e)
                });
            self
        }
        pub fn script_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .script_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for script_code: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesLocale> for super::StylesLocale {
        type Error = String;
        fn try_from(value: StylesLocale) -> Result<Self, String> {
            Ok(Self {
                country_code: value.country_code?,
                language_code: value.language_code?,
                script_code: value.script_code?,
            })
        }
    }
    impl From<super::StylesLocale> for StylesLocale {
        fn from(value: super::StylesLocale) -> Self {
            Self {
                country_code: Ok(value.country_code),
                language_code: Ok(value.language_code),
                script_code: Ok(value.script_code),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesOffset {
        dx: Result<Option<f64>, String>,
        dy: Result<Option<f64>, String>,
    }
    impl Default for StylesOffset {
        fn default() -> Self {
            Self {
                dx: Ok(Default::default()),
                dy: Ok(Default::default()),
            }
        }
    }
    impl StylesOffset {
        pub fn dx<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .dx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dx: {}", e));
            self
        }
        pub fn dy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .dy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dy: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StylesOffset> for super::StylesOffset {
        type Error = String;
        fn try_from(value: StylesOffset) -> Result<Self, String> {
            Ok(Self {
                dx: value.dx?,
                dy: value.dy?,
            })
        }
    }
    impl From<super::StylesOffset> for StylesOffset {
        fn from(value: super::StylesOffset) -> Self {
            Self {
                dx: Ok(value.dx),
                dy: Ok(value.dy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesOutlinedBorder {
        side: Result<Option<super::StylesBorderSide>, String>,
    }
    impl Default for StylesOutlinedBorder {
        fn default() -> Self {
            Self {
                side: Ok(Default::default()),
            }
        }
    }
    impl StylesOutlinedBorder {
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBorderSide>>,
            T::Error: std::fmt::Display,
        {
            self
                .side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StylesOutlinedBorder> for super::StylesOutlinedBorder {
        type Error = String;
        fn try_from(value: StylesOutlinedBorder) -> Result<Self, String> {
            Ok(Self { side: value.side? })
        }
    }
    impl From<super::StylesOutlinedBorder> for StylesOutlinedBorder {
        fn from(value: super::StylesOutlinedBorder) -> Self {
            Self { side: Ok(value.side) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesPadding {
        bottom: Result<Option<f64>, String>,
        left: Result<Option<f64>, String>,
        right: Result<Option<f64>, String>,
        top: Result<Option<f64>, String>,
    }
    impl Default for StylesPadding {
        fn default() -> Self {
            Self {
                bottom: Ok(Default::default()),
                left: Ok(Default::default()),
                right: Ok(Default::default()),
                top: Ok(Default::default()),
            }
        }
    }
    impl StylesPadding {
        pub fn bottom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .bottom = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bottom: {}", e)
                });
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .right = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for right: {}", e)
                });
            self
        }
        pub fn top<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .top = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StylesPadding> for super::StylesPadding {
        type Error = String;
        fn try_from(value: StylesPadding) -> Result<Self, String> {
            Ok(Self {
                bottom: value.bottom?,
                left: value.left?,
                right: value.right?,
                top: value.top?,
            })
        }
    }
    impl From<super::StylesPadding> for StylesPadding {
        fn from(value: super::StylesPadding) -> Self {
            Self {
                bottom: Ok(value.bottom),
                left: Ok(value.left),
                right: Ok(value.right),
                top: Ok(value.top),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesRadioStyle {
        active_color: Result<Option<super::StylesColor>, String>,
        focus_color: Result<Option<super::StylesColor>, String>,
        hovercolor: Result<Option<super::StylesColor>, String>,
        splash_radius: Result<Option<f64>, String>,
        unselected_color: Result<Option<super::StylesColor>, String>,
        visual_density: Result<Option<super::StylesVisualDensity>, String>,
    }
    impl Default for StylesRadioStyle {
        fn default() -> Self {
            Self {
                active_color: Ok(Default::default()),
                focus_color: Ok(Default::default()),
                hovercolor: Ok(Default::default()),
                splash_radius: Ok(Default::default()),
                unselected_color: Ok(Default::default()),
                visual_density: Ok(Default::default()),
            }
        }
    }
    impl StylesRadioStyle {
        pub fn active_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .active_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for active_color: {}", e)
                });
            self
        }
        pub fn focus_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .focus_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for focus_color: {}", e)
                });
            self
        }
        pub fn hovercolor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .hovercolor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hovercolor: {}", e)
                });
            self
        }
        pub fn splash_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .splash_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for splash_radius: {}", e)
                });
            self
        }
        pub fn unselected_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .unselected_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for unselected_color: {}", e
                    )
                });
            self
        }
        pub fn visual_density<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesVisualDensity>>,
            T::Error: std::fmt::Display,
        {
            self
                .visual_density = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for visual_density: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesRadioStyle> for super::StylesRadioStyle {
        type Error = String;
        fn try_from(value: StylesRadioStyle) -> Result<Self, String> {
            Ok(Self {
                active_color: value.active_color?,
                focus_color: value.focus_color?,
                hovercolor: value.hovercolor?,
                splash_radius: value.splash_radius?,
                unselected_color: value.unselected_color?,
                visual_density: value.visual_density?,
            })
        }
    }
    impl From<super::StylesRadioStyle> for StylesRadioStyle {
        fn from(value: super::StylesRadioStyle) -> Self {
            Self {
                active_color: Ok(value.active_color),
                focus_color: Ok(value.focus_color),
                hovercolor: Ok(value.hovercolor),
                splash_radius: Ok(value.splash_radius),
                unselected_color: Ok(value.unselected_color),
                visual_density: Ok(value.visual_density),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesRadius {
        x: Result<Option<f64>, String>,
        y: Result<Option<f64>, String>,
    }
    impl Default for StylesRadius {
        fn default() -> Self {
            Self {
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl StylesRadius {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StylesRadius> for super::StylesRadius {
        type Error = String;
        fn try_from(value: StylesRadius) -> Result<Self, String> {
            Ok(Self { x: value.x?, y: value.y? })
        }
    }
    impl From<super::StylesRadius> for StylesRadius {
        fn from(value: super::StylesRadius) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesRect {
        height: Result<Option<f64>, String>,
        left: Result<Option<f64>, String>,
        top: Result<Option<f64>, String>,
        width: Result<Option<f64>, String>,
    }
    impl Default for StylesRect {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                left: Ok(Default::default()),
                top: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl StylesRect {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn top<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .top = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top: {}", e));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for width: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesRect> for super::StylesRect {
        type Error = String;
        fn try_from(value: StylesRect) -> Result<Self, String> {
            Ok(Self {
                height: value.height?,
                left: value.left?,
                top: value.top?,
                width: value.width?,
            })
        }
    }
    impl From<super::StylesRect> for StylesRect {
        fn from(value: super::StylesRect) -> Self {
            Self {
                height: Ok(value.height),
                left: Ok(value.left),
                top: Ok(value.top),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesSliderStyle {
        active_color: Result<Option<super::StylesColor>, String>,
        inactive_color: Result<Option<super::StylesColor>, String>,
        thumb_color: Result<Option<super::StylesColor>, String>,
    }
    impl Default for StylesSliderStyle {
        fn default() -> Self {
            Self {
                active_color: Ok(Default::default()),
                inactive_color: Ok(Default::default()),
                thumb_color: Ok(Default::default()),
            }
        }
    }
    impl StylesSliderStyle {
        pub fn active_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .active_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for active_color: {}", e)
                });
            self
        }
        pub fn inactive_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .inactive_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for inactive_color: {}", e)
                });
            self
        }
        pub fn thumb_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .thumb_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for thumb_color: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesSliderStyle> for super::StylesSliderStyle {
        type Error = String;
        fn try_from(value: StylesSliderStyle) -> Result<Self, String> {
            Ok(Self {
                active_color: value.active_color?,
                inactive_color: value.inactive_color?,
                thumb_color: value.thumb_color?,
            })
        }
    }
    impl From<super::StylesSliderStyle> for StylesSliderStyle {
        fn from(value: super::StylesSliderStyle) -> Self {
            Self {
                active_color: Ok(value.active_color),
                inactive_color: Ok(value.inactive_color),
                thumb_color: Ok(value.thumb_color),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesStrutStyle {
        debug_label: Result<Option<String>, String>,
        font_family: Result<Option<String>, String>,
        font_family_fallback: Result<Vec<serde_json::Value>, String>,
        font_size: Result<Option<f64>, String>,
        font_weight: Result<Option<String>, String>,
        force_strut_height: Result<Option<bool>, String>,
        height: Result<Option<f64>, String>,
        leading: Result<Option<f64>, String>,
        leading_distribution: Result<
            Option<super::StylesTextLeadingDistribution>,
            String,
        >,
    }
    impl Default for StylesStrutStyle {
        fn default() -> Self {
            Self {
                debug_label: Ok(Default::default()),
                font_family: Ok(Default::default()),
                font_family_fallback: Ok(Default::default()),
                font_size: Ok(Default::default()),
                font_weight: Ok(Default::default()),
                force_strut_height: Ok(Default::default()),
                height: Ok(Default::default()),
                leading: Ok(Default::default()),
                leading_distribution: Ok(Default::default()),
            }
        }
    }
    impl StylesStrutStyle {
        pub fn debug_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .debug_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for debug_label: {}", e)
                });
            self
        }
        pub fn font_family<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_family = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_family: {}", e)
                });
            self
        }
        pub fn font_family_fallback<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_family_fallback = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for font_family_fallback: {}", e
                    )
                });
            self
        }
        pub fn font_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_size = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_size: {}", e)
                });
            self
        }
        pub fn font_weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_weight = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_weight: {}", e)
                });
            self
        }
        pub fn force_strut_height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .force_strut_height = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for force_strut_height: {}", e
                    )
                });
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn leading<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .leading = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for leading: {}", e)
                });
            self
        }
        pub fn leading_distribution<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextLeadingDistribution>>,
            T::Error: std::fmt::Display,
        {
            self
                .leading_distribution = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for leading_distribution: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesStrutStyle> for super::StylesStrutStyle {
        type Error = String;
        fn try_from(value: StylesStrutStyle) -> Result<Self, String> {
            Ok(Self {
                debug_label: value.debug_label?,
                font_family: value.font_family?,
                font_family_fallback: value.font_family_fallback?,
                font_size: value.font_size?,
                font_weight: value.font_weight?,
                force_strut_height: value.force_strut_height?,
                height: value.height?,
                leading: value.leading?,
                leading_distribution: value.leading_distribution?,
            })
        }
    }
    impl From<super::StylesStrutStyle> for StylesStrutStyle {
        fn from(value: super::StylesStrutStyle) -> Self {
            Self {
                debug_label: Ok(value.debug_label),
                font_family: Ok(value.font_family),
                font_family_fallback: Ok(value.font_family_fallback),
                font_size: Ok(value.font_size),
                font_weight: Ok(value.font_weight),
                force_strut_height: Ok(value.force_strut_height),
                height: Ok(value.height),
                leading: Ok(value.leading),
                leading_distribution: Ok(value.leading_distribution),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesTextFieldStyle {
        cursor_color: Result<Option<super::StylesColor>, String>,
        cursor_height: Result<Option<f64>, String>,
        cursor_radius: Result<Option<super::StylesRadius>, String>,
        cursor_width: Result<Option<f64>, String>,
        decoration: Result<Option<super::StylesInputDecoration>, String>,
        keyboard_appearance: Result<Option<super::StylesBrightness>, String>,
        obscuring_character: Result<Option<String>, String>,
        scroll_padding: Result<Option<super::StylesPadding>, String>,
        selection_height_style: Result<Option<super::StylesBoxHeightStyle>, String>,
        selection_width_style: Result<Option<super::StylesBoxWidthStyle>, String>,
        strut_style: Result<Option<super::StylesStrutStyle>, String>,
        text_align: Result<Option<super::StylesTextAlign>, String>,
        text_align_vertical: Result<Option<super::StylesTextAlignVertical>, String>,
        text_style: Result<Option<super::StylesTextStyle>, String>,
    }
    impl Default for StylesTextFieldStyle {
        fn default() -> Self {
            Self {
                cursor_color: Ok(Default::default()),
                cursor_height: Ok(Default::default()),
                cursor_radius: Ok(Default::default()),
                cursor_width: Ok(Default::default()),
                decoration: Ok(Default::default()),
                keyboard_appearance: Ok(Default::default()),
                obscuring_character: Ok(Default::default()),
                scroll_padding: Ok(Default::default()),
                selection_height_style: Ok(Default::default()),
                selection_width_style: Ok(Default::default()),
                strut_style: Ok(Default::default()),
                text_align: Ok(Default::default()),
                text_align_vertical: Ok(Default::default()),
                text_style: Ok(Default::default()),
            }
        }
    }
    impl StylesTextFieldStyle {
        pub fn cursor_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .cursor_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cursor_color: {}", e)
                });
            self
        }
        pub fn cursor_height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .cursor_height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cursor_height: {}", e)
                });
            self
        }
        pub fn cursor_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesRadius>>,
            T::Error: std::fmt::Display,
        {
            self
                .cursor_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cursor_radius: {}", e)
                });
            self
        }
        pub fn cursor_width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .cursor_width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cursor_width: {}", e)
                });
            self
        }
        pub fn decoration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesInputDecoration>>,
            T::Error: std::fmt::Display,
        {
            self
                .decoration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decoration: {}", e)
                });
            self
        }
        pub fn keyboard_appearance<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBrightness>>,
            T::Error: std::fmt::Display,
        {
            self
                .keyboard_appearance = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for keyboard_appearance: {}", e
                    )
                });
            self
        }
        pub fn obscuring_character<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .obscuring_character = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for obscuring_character: {}", e
                    )
                });
            self
        }
        pub fn scroll_padding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesPadding>>,
            T::Error: std::fmt::Display,
        {
            self
                .scroll_padding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scroll_padding: {}", e)
                });
            self
        }
        pub fn selection_height_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxHeightStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .selection_height_style = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for selection_height_style: {}",
                        e
                    )
                });
            self
        }
        pub fn selection_width_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesBoxWidthStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .selection_width_style = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for selection_width_style: {}",
                        e
                    )
                });
            self
        }
        pub fn strut_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesStrutStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .strut_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for strut_style: {}", e)
                });
            self
        }
        pub fn text_align<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextAlign>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_align = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for text_align: {}", e)
                });
            self
        }
        pub fn text_align_vertical<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextAlignVertical>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_align_vertical = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for text_align_vertical: {}", e
                    )
                });
            self
        }
        pub fn text_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for text_style: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesTextFieldStyle> for super::StylesTextFieldStyle {
        type Error = String;
        fn try_from(value: StylesTextFieldStyle) -> Result<Self, String> {
            Ok(Self {
                cursor_color: value.cursor_color?,
                cursor_height: value.cursor_height?,
                cursor_radius: value.cursor_radius?,
                cursor_width: value.cursor_width?,
                decoration: value.decoration?,
                keyboard_appearance: value.keyboard_appearance?,
                obscuring_character: value.obscuring_character?,
                scroll_padding: value.scroll_padding?,
                selection_height_style: value.selection_height_style?,
                selection_width_style: value.selection_width_style?,
                strut_style: value.strut_style?,
                text_align: value.text_align?,
                text_align_vertical: value.text_align_vertical?,
                text_style: value.text_style?,
            })
        }
    }
    impl From<super::StylesTextFieldStyle> for StylesTextFieldStyle {
        fn from(value: super::StylesTextFieldStyle) -> Self {
            Self {
                cursor_color: Ok(value.cursor_color),
                cursor_height: Ok(value.cursor_height),
                cursor_radius: Ok(value.cursor_radius),
                cursor_width: Ok(value.cursor_width),
                decoration: Ok(value.decoration),
                keyboard_appearance: Ok(value.keyboard_appearance),
                obscuring_character: Ok(value.obscuring_character),
                scroll_padding: Ok(value.scroll_padding),
                selection_height_style: Ok(value.selection_height_style),
                selection_width_style: Ok(value.selection_width_style),
                strut_style: Ok(value.strut_style),
                text_align: Ok(value.text_align),
                text_align_vertical: Ok(value.text_align_vertical),
                text_style: Ok(value.text_style),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesTextInputType {
        copy: Result<Option<bool>, String>,
        cut: Result<Option<bool>, String>,
        paste: Result<Option<bool>, String>,
        select_all: Result<Option<bool>, String>,
    }
    impl Default for StylesTextInputType {
        fn default() -> Self {
            Self {
                copy: Ok(Default::default()),
                cut: Ok(Default::default()),
                paste: Ok(Default::default()),
                select_all: Ok(Default::default()),
            }
        }
    }
    impl StylesTextInputType {
        pub fn copy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .copy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for copy: {}", e));
            self
        }
        pub fn cut<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .cut = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cut: {}", e));
            self
        }
        pub fn paste<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .paste = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for paste: {}", e)
                });
            self
        }
        pub fn select_all<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .select_all = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for select_all: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesTextInputType> for super::StylesTextInputType {
        type Error = String;
        fn try_from(value: StylesTextInputType) -> Result<Self, String> {
            Ok(Self {
                copy: value.copy?,
                cut: value.cut?,
                paste: value.paste?,
                select_all: value.select_all?,
            })
        }
    }
    impl From<super::StylesTextInputType> for StylesTextInputType {
        fn from(value: super::StylesTextInputType) -> Self {
            Self {
                copy: Ok(value.copy),
                cut: Ok(value.cut),
                paste: Ok(value.paste),
                select_all: Ok(value.select_all),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesTextStyle {
        color: Result<Option<super::StylesColor>, String>,
        decoration: Result<Option<super::StylesTextDecoration>, String>,
        decoration_color: Result<Option<super::StylesColor>, String>,
        decoration_style: Result<Option<super::StylesTextDecorationStyle>, String>,
        decoration_thickness: Result<Option<f64>, String>,
        font_family: Result<Option<String>, String>,
        font_family_fallback: Result<Vec<String>, String>,
        font_size: Result<Option<f64>, String>,
        font_style: Result<Option<super::StylesTextStyleFontStyle>, String>,
        font_weight: Result<Option<super::StylesTextStyleFontWeight>, String>,
        height: Result<Option<f64>, String>,
        letter_spacing: Result<Option<f64>, String>,
        overflow: Result<Option<super::StylesTextStyleOverflow>, String>,
        shadows: Result<Vec<super::StylesBoxShadow>, String>,
        text_baseline: Result<Option<super::StylesTextBaseline>, String>,
        word_spacing: Result<Option<f64>, String>,
    }
    impl Default for StylesTextStyle {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                decoration: Ok(Default::default()),
                decoration_color: Ok(Default::default()),
                decoration_style: Ok(Default::default()),
                decoration_thickness: Ok(Default::default()),
                font_family: Ok(Default::default()),
                font_family_fallback: Ok(Default::default()),
                font_size: Ok(Default::default()),
                font_style: Ok(Default::default()),
                font_weight: Ok(Default::default()),
                height: Ok(Default::default()),
                letter_spacing: Ok(Default::default()),
                overflow: Ok(Default::default()),
                shadows: Ok(Default::default()),
                text_baseline: Ok(Default::default()),
                word_spacing: Ok(Default::default()),
            }
        }
    }
    impl StylesTextStyle {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn decoration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextDecoration>>,
            T::Error: std::fmt::Display,
        {
            self
                .decoration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decoration: {}", e)
                });
            self
        }
        pub fn decoration_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .decoration_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for decoration_color: {}", e
                    )
                });
            self
        }
        pub fn decoration_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextDecorationStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .decoration_style = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for decoration_style: {}", e
                    )
                });
            self
        }
        pub fn decoration_thickness<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .decoration_thickness = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for decoration_thickness: {}", e
                    )
                });
            self
        }
        pub fn font_family<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_family = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_family: {}", e)
                });
            self
        }
        pub fn font_family_fallback<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_family_fallback = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for font_family_fallback: {}", e
                    )
                });
            self
        }
        pub fn font_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_size = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_size: {}", e)
                });
            self
        }
        pub fn font_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyleFontStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_style: {}", e)
                });
            self
        }
        pub fn font_weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyleFontWeight>>,
            T::Error: std::fmt::Display,
        {
            self
                .font_weight = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for font_weight: {}", e)
                });
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn letter_spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .letter_spacing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for letter_spacing: {}", e)
                });
            self
        }
        pub fn overflow<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyleOverflow>>,
            T::Error: std::fmt::Display,
        {
            self
                .overflow = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for overflow: {}", e)
                });
            self
        }
        pub fn shadows<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::StylesBoxShadow>>,
            T::Error: std::fmt::Display,
        {
            self
                .shadows = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for shadows: {}", e)
                });
            self
        }
        pub fn text_baseline<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextBaseline>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_baseline = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for text_baseline: {}", e)
                });
            self
        }
        pub fn word_spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .word_spacing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for word_spacing: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesTextStyle> for super::StylesTextStyle {
        type Error = String;
        fn try_from(value: StylesTextStyle) -> Result<Self, String> {
            Ok(Self {
                color: value.color?,
                decoration: value.decoration?,
                decoration_color: value.decoration_color?,
                decoration_style: value.decoration_style?,
                decoration_thickness: value.decoration_thickness?,
                font_family: value.font_family?,
                font_family_fallback: value.font_family_fallback?,
                font_size: value.font_size?,
                font_style: value.font_style?,
                font_weight: value.font_weight?,
                height: value.height?,
                letter_spacing: value.letter_spacing?,
                overflow: value.overflow?,
                shadows: value.shadows?,
                text_baseline: value.text_baseline?,
                word_spacing: value.word_spacing?,
            })
        }
    }
    impl From<super::StylesTextStyle> for StylesTextStyle {
        fn from(value: super::StylesTextStyle) -> Self {
            Self {
                color: Ok(value.color),
                decoration: Ok(value.decoration),
                decoration_color: Ok(value.decoration_color),
                decoration_style: Ok(value.decoration_style),
                decoration_thickness: Ok(value.decoration_thickness),
                font_family: Ok(value.font_family),
                font_family_fallback: Ok(value.font_family_fallback),
                font_size: Ok(value.font_size),
                font_style: Ok(value.font_style),
                font_weight: Ok(value.font_weight),
                height: Ok(value.height),
                letter_spacing: Ok(value.letter_spacing),
                overflow: Ok(value.overflow),
                shadows: Ok(value.shadows),
                text_baseline: Ok(value.text_baseline),
                word_spacing: Ok(value.word_spacing),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesToggleStyle {
        active_color: Result<Option<super::StylesColor>, String>,
        active_thumb_image: Result<Option<super::Image>, String>,
        active_track_color: Result<Option<super::StylesColor>, String>,
        focus_color: Result<Option<super::StylesColor>, String>,
        hover_color: Result<Option<super::StylesColor>, String>,
        inactive_thumb_color: Result<Option<super::StylesColor>, String>,
        inactive_thumb_image: Result<Option<super::Image>, String>,
        inactive_track_color: Result<Option<super::StylesColor>, String>,
        material_tap_target_size: Result<
            Option<super::StylesToggleStyleMaterialTapTargetSize>,
            String,
        >,
    }
    impl Default for StylesToggleStyle {
        fn default() -> Self {
            Self {
                active_color: Ok(Default::default()),
                active_thumb_image: Ok(Default::default()),
                active_track_color: Ok(Default::default()),
                focus_color: Ok(Default::default()),
                hover_color: Ok(Default::default()),
                inactive_thumb_color: Ok(Default::default()),
                inactive_thumb_image: Ok(Default::default()),
                inactive_track_color: Ok(Default::default()),
                material_tap_target_size: Ok(Default::default()),
            }
        }
    }
    impl StylesToggleStyle {
        pub fn active_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .active_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for active_color: {}", e)
                });
            self
        }
        pub fn active_thumb_image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self
                .active_thumb_image = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for active_thumb_image: {}", e
                    )
                });
            self
        }
        pub fn active_track_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .active_track_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for active_track_color: {}", e
                    )
                });
            self
        }
        pub fn focus_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .focus_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for focus_color: {}", e)
                });
            self
        }
        pub fn hover_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .hover_color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hover_color: {}", e)
                });
            self
        }
        pub fn inactive_thumb_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .inactive_thumb_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for inactive_thumb_color: {}", e
                    )
                });
            self
        }
        pub fn inactive_thumb_image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self
                .inactive_thumb_image = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for inactive_thumb_image: {}", e
                    )
                });
            self
        }
        pub fn inactive_track_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesColor>>,
            T::Error: std::fmt::Display,
        {
            self
                .inactive_track_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for inactive_track_color: {}", e
                    )
                });
            self
        }
        pub fn material_tap_target_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::StylesToggleStyleMaterialTapTargetSize>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .material_tap_target_size = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for material_tap_target_size: {}",
                        e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesToggleStyle> for super::StylesToggleStyle {
        type Error = String;
        fn try_from(value: StylesToggleStyle) -> Result<Self, String> {
            Ok(Self {
                active_color: value.active_color?,
                active_thumb_image: value.active_thumb_image?,
                active_track_color: value.active_track_color?,
                focus_color: value.focus_color?,
                hover_color: value.hover_color?,
                inactive_thumb_color: value.inactive_thumb_color?,
                inactive_thumb_image: value.inactive_thumb_image?,
                inactive_track_color: value.inactive_track_color?,
                material_tap_target_size: value.material_tap_target_size?,
            })
        }
    }
    impl From<super::StylesToggleStyle> for StylesToggleStyle {
        fn from(value: super::StylesToggleStyle) -> Self {
            Self {
                active_color: Ok(value.active_color),
                active_thumb_image: Ok(value.active_thumb_image),
                active_track_color: Ok(value.active_track_color),
                focus_color: Ok(value.focus_color),
                hover_color: Ok(value.hover_color),
                inactive_thumb_color: Ok(value.inactive_thumb_color),
                inactive_thumb_image: Ok(value.inactive_thumb_image),
                inactive_track_color: Ok(value.inactive_track_color),
                material_tap_target_size: Ok(value.material_tap_target_size),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StylesToolbarOptions {
        decimal: Result<Option<bool>, String>,
        signed: Result<Option<bool>, String>,
    }
    impl Default for StylesToolbarOptions {
        fn default() -> Self {
            Self {
                decimal: Ok(Default::default()),
                signed: Ok(Default::default()),
            }
        }
    }
    impl StylesToolbarOptions {
        pub fn decimal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .decimal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decimal: {}", e)
                });
            self
        }
        pub fn signed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .signed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for signed: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<StylesToolbarOptions> for super::StylesToolbarOptions {
        type Error = String;
        fn try_from(value: StylesToolbarOptions) -> Result<Self, String> {
            Ok(Self {
                decimal: value.decimal?,
                signed: value.signed?,
            })
        }
    }
    impl From<super::StylesToolbarOptions> for StylesToolbarOptions {
        fn from(value: super::StylesToolbarOptions) -> Self {
            Self {
                decimal: Ok(value.decimal),
                signed: Ok(value.signed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Text {
        children: Result<Vec<super::Text>, String>,
        locale: Result<Option<super::StylesLocale>, String>,
        semantics_label: Result<Option<String>, String>,
        spell_out: Result<Option<bool>, String>,
        style: Result<Option<super::StylesTextStyle>, String>,
        text_align: Result<Option<super::TextTextAlign>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<String, String>,
    }
    impl Default for Text {
        fn default() -> Self {
            Self {
                children: Ok(Default::default()),
                locale: Ok(Default::default()),
                semantics_label: Ok(Default::default()),
                spell_out: Ok(Default::default()),
                style: Ok(Default::default()),
                text_align: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Text {
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Text>>,
            T::Error: std::fmt::Display,
        {
            self
                .children = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for children: {}", e)
                });
            self
        }
        pub fn locale<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesLocale>>,
            T::Error: std::fmt::Display,
        {
            self
                .locale = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for locale: {}", e)
                });
            self
        }
        pub fn semantics_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .semantics_label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for semantics_label: {}", e)
                });
            self
        }
        pub fn spell_out<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .spell_out = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for spell_out: {}", e)
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn text_align<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TextTextAlign>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_align = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for text_align: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Text> for super::Text {
        type Error = String;
        fn try_from(value: Text) -> Result<Self, String> {
            Ok(Self {
                children: value.children?,
                locale: value.locale?,
                semantics_label: value.semantics_label?,
                spell_out: value.spell_out?,
                style: value.style?,
                text_align: value.text_align?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Text> for Text {
        fn from(value: super::Text) -> Self {
            Self {
                children: Ok(value.children),
                locale: Ok(value.locale),
                semantics_label: Ok(value.semantics_label),
                spell_out: Ok(value.spell_out),
                style: Ok(value.style),
                text_align: Ok(value.text_align),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Textfield {
        autocorrect: Result<Option<bool>, String>,
        autofill_hints: Result<Option<super::StylesAutofillHints>, String>,
        autofocus: Result<Option<bool>, String>,
        build_counter: Result<Option<super::Listener>, String>,
        drag_start_behavior: Result<Option<super::StylesDragStartBehavior>, String>,
        enable_interactive_selection: Result<Option<bool>, String>,
        enabled: Result<Option<bool>, String>,
        expands: Result<Option<bool>, String>,
        keyboard_type: Result<Option<super::StylesTextInputType>, String>,
        max_length: Result<Option<i64>, String>,
        max_length_enforcement: Result<
            Option<super::StylesMaxLengthEnforcement>,
            String,
        >,
        max_lines: Result<Option<i64>, String>,
        min_lines: Result<Option<i64>, String>,
        name: Result<Option<String>, String>,
        obscure_text: Result<Option<bool>, String>,
        on_app_private_command: Result<Option<super::Listener>, String>,
        on_changed: Result<Option<super::Listener>, String>,
        on_editing_complete: Result<Option<super::Listener>, String>,
        on_submitted: Result<Option<super::Listener>, String>,
        on_tap: Result<Option<super::Listener>, String>,
        read_only: Result<Option<bool>, String>,
        show_cursor: Result<Option<bool>, String>,
        style: Result<Option<super::StylesTextFieldStyle>, String>,
        text_capitalization: Result<Option<super::StylesTextCapitalization>, String>,
        text_direction: Result<Option<super::StylesTextDirection>, String>,
        text_input_action: Result<Option<super::StylesTextInputAction>, String>,
        toolbar_options: Result<Option<super::StylesToolbarOptions>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<String, String>,
    }
    impl Default for Textfield {
        fn default() -> Self {
            Self {
                autocorrect: Ok(Default::default()),
                autofill_hints: Ok(Default::default()),
                autofocus: Ok(Default::default()),
                build_counter: Ok(Default::default()),
                drag_start_behavior: Ok(Default::default()),
                enable_interactive_selection: Ok(Default::default()),
                enabled: Ok(Default::default()),
                expands: Ok(Default::default()),
                keyboard_type: Ok(Default::default()),
                max_length: Ok(Default::default()),
                max_length_enforcement: Ok(Default::default()),
                max_lines: Ok(Default::default()),
                min_lines: Ok(Default::default()),
                name: Ok(Default::default()),
                obscure_text: Ok(Default::default()),
                on_app_private_command: Ok(Default::default()),
                on_changed: Ok(Default::default()),
                on_editing_complete: Ok(Default::default()),
                on_submitted: Ok(Default::default()),
                on_tap: Ok(Default::default()),
                read_only: Ok(Default::default()),
                show_cursor: Ok(Default::default()),
                style: Ok(Default::default()),
                text_capitalization: Ok(Default::default()),
                text_direction: Ok(Default::default()),
                text_input_action: Ok(Default::default()),
                toolbar_options: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Textfield {
        pub fn autocorrect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .autocorrect = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autocorrect: {}", e)
                });
            self
        }
        pub fn autofill_hints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesAutofillHints>>,
            T::Error: std::fmt::Display,
        {
            self
                .autofill_hints = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autofill_hints: {}", e)
                });
            self
        }
        pub fn autofocus<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .autofocus = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autofocus: {}", e)
                });
            self
        }
        pub fn build_counter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .build_counter = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for build_counter: {}", e)
                });
            self
        }
        pub fn drag_start_behavior<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesDragStartBehavior>>,
            T::Error: std::fmt::Display,
        {
            self
                .drag_start_behavior = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for drag_start_behavior: {}", e
                    )
                });
            self
        }
        pub fn enable_interactive_selection<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .enable_interactive_selection = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for enable_interactive_selection: {}",
                        e
                    )
                });
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .enabled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for enabled: {}", e)
                });
            self
        }
        pub fn expands<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .expands = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expands: {}", e)
                });
            self
        }
        pub fn keyboard_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextInputType>>,
            T::Error: std::fmt::Display,
        {
            self
                .keyboard_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for keyboard_type: {}", e)
                });
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .max_length = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for max_length: {}", e)
                });
            self
        }
        pub fn max_length_enforcement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesMaxLengthEnforcement>>,
            T::Error: std::fmt::Display,
        {
            self
                .max_length_enforcement = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for max_length_enforcement: {}",
                        e
                    )
                });
            self
        }
        pub fn max_lines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .max_lines = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for max_lines: {}", e)
                });
            self
        }
        pub fn min_lines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self
                .min_lines = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for min_lines: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn obscure_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .obscure_text = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for obscure_text: {}", e)
                });
            self
        }
        pub fn on_app_private_command<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_app_private_command = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for on_app_private_command: {}",
                        e
                    )
                });
            self
        }
        pub fn on_changed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_changed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_changed: {}", e)
                });
            self
        }
        pub fn on_editing_complete<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_editing_complete = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for on_editing_complete: {}", e
                    )
                });
            self
        }
        pub fn on_submitted<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_submitted = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_submitted: {}", e)
                });
            self
        }
        pub fn on_tap<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_tap = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_tap: {}", e)
                });
            self
        }
        pub fn read_only<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .read_only = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for read_only: {}", e)
                });
            self
        }
        pub fn show_cursor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .show_cursor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for show_cursor: {}", e)
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextFieldStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn text_capitalization<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextCapitalization>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_capitalization = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for text_capitalization: {}", e
                    )
                });
            self
        }
        pub fn text_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_direction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for text_direction: {}", e)
                });
            self
        }
        pub fn text_input_action<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextInputAction>>,
            T::Error: std::fmt::Display,
        {
            self
                .text_input_action = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for text_input_action: {}", e
                    )
                });
            self
        }
        pub fn toolbar_options<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesToolbarOptions>>,
            T::Error: std::fmt::Display,
        {
            self
                .toolbar_options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for toolbar_options: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Textfield> for super::Textfield {
        type Error = String;
        fn try_from(value: Textfield) -> Result<Self, String> {
            Ok(Self {
                autocorrect: value.autocorrect?,
                autofill_hints: value.autofill_hints?,
                autofocus: value.autofocus?,
                build_counter: value.build_counter?,
                drag_start_behavior: value.drag_start_behavior?,
                enable_interactive_selection: value.enable_interactive_selection?,
                enabled: value.enabled?,
                expands: value.expands?,
                keyboard_type: value.keyboard_type?,
                max_length: value.max_length?,
                max_length_enforcement: value.max_length_enforcement?,
                max_lines: value.max_lines?,
                min_lines: value.min_lines?,
                name: value.name?,
                obscure_text: value.obscure_text?,
                on_app_private_command: value.on_app_private_command?,
                on_changed: value.on_changed?,
                on_editing_complete: value.on_editing_complete?,
                on_submitted: value.on_submitted?,
                on_tap: value.on_tap?,
                read_only: value.read_only?,
                show_cursor: value.show_cursor?,
                style: value.style?,
                text_capitalization: value.text_capitalization?,
                text_direction: value.text_direction?,
                text_input_action: value.text_input_action?,
                toolbar_options: value.toolbar_options?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Textfield> for Textfield {
        fn from(value: super::Textfield) -> Self {
            Self {
                autocorrect: Ok(value.autocorrect),
                autofill_hints: Ok(value.autofill_hints),
                autofocus: Ok(value.autofocus),
                build_counter: Ok(value.build_counter),
                drag_start_behavior: Ok(value.drag_start_behavior),
                enable_interactive_selection: Ok(value.enable_interactive_selection),
                enabled: Ok(value.enabled),
                expands: Ok(value.expands),
                keyboard_type: Ok(value.keyboard_type),
                max_length: Ok(value.max_length),
                max_length_enforcement: Ok(value.max_length_enforcement),
                max_lines: Ok(value.max_lines),
                min_lines: Ok(value.min_lines),
                name: Ok(value.name),
                obscure_text: Ok(value.obscure_text),
                on_app_private_command: Ok(value.on_app_private_command),
                on_changed: Ok(value.on_changed),
                on_editing_complete: Ok(value.on_editing_complete),
                on_submitted: Ok(value.on_submitted),
                on_tap: Ok(value.on_tap),
                read_only: Ok(value.read_only),
                show_cursor: Ok(value.show_cursor),
                style: Ok(value.style),
                text_capitalization: Ok(value.text_capitalization),
                text_direction: Ok(value.text_direction),
                text_input_action: Ok(value.text_input_action),
                toolbar_options: Ok(value.toolbar_options),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Toggle {
        autofocus: Result<Option<bool>, String>,
        disabled: Result<Option<bool>, String>,
        drag_start_behavior: Result<Option<super::ToggleDragStartBehavior>, String>,
        name: Result<Option<String>, String>,
        on_pressed: Result<Option<super::Listener>, String>,
        splash_radius: Result<Option<f64>, String>,
        style: Result<Option<super::StylesToggleStyle>, String>,
        type_: Result<serde_json::Value, String>,
        value: Result<bool, String>,
    }
    impl Default for Toggle {
        fn default() -> Self {
            Self {
                autofocus: Ok(Default::default()),
                disabled: Ok(Default::default()),
                drag_start_behavior: Ok(Default::default()),
                name: Ok(Default::default()),
                on_pressed: Ok(Default::default()),
                splash_radius: Ok(Default::default()),
                style: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Toggle {
        pub fn autofocus<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .autofocus = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autofocus: {}", e)
                });
            self
        }
        pub fn disabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self
                .disabled = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for disabled: {}", e)
                });
            self
        }
        pub fn drag_start_behavior<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ToggleDragStartBehavior>>,
            T::Error: std::fmt::Display,
        {
            self
                .drag_start_behavior = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for drag_start_behavior: {}", e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn on_pressed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Listener>>,
            T::Error: std::fmt::Display,
        {
            self
                .on_pressed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for on_pressed: {}", e)
                });
            self
        }
        pub fn splash_radius<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .splash_radius = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for splash_radius: {}", e)
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesToggleStyle>>,
            T::Error: std::fmt::Display,
        {
            self
                .style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self
                .value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Toggle> for super::Toggle {
        type Error = String;
        fn try_from(value: Toggle) -> Result<Self, String> {
            Ok(Self {
                autofocus: value.autofocus?,
                disabled: value.disabled?,
                drag_start_behavior: value.drag_start_behavior?,
                name: value.name?,
                on_pressed: value.on_pressed?,
                splash_radius: value.splash_radius?,
                style: value.style?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Toggle> for Toggle {
        fn from(value: super::Toggle) -> Self {
            Self {
                autofocus: Ok(value.autofocus),
                disabled: Ok(value.disabled),
                drag_start_behavior: Ok(value.drag_start_behavior),
                name: Ok(value.name),
                on_pressed: Ok(value.on_pressed),
                splash_radius: Ok(value.splash_radius),
                style: Ok(value.style),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct View {
        context: Result<serde_json::Map<String, serde_json::Value>, String>,
        find: Result<Option<super::ViewDefinitionsFind>, String>,
        name: Result<String, String>,
        props: Result<Option<super::DefsProps>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for View {
        fn default() -> Self {
            Self {
                context: Ok(Default::default()),
                find: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                props: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl View {
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self
                .context = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for context: {}", e)
                });
            self
        }
        pub fn find<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ViewDefinitionsFind>>,
            T::Error: std::fmt::Display,
        {
            self
                .find = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for find: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn props<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DefsProps>>,
            T::Error: std::fmt::Display,
        {
            self
                .props = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for props: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<View> for super::View {
        type Error = String;
        fn try_from(value: View) -> Result<Self, String> {
            Ok(Self {
                context: value.context?,
                find: value.find?,
                name: value.name?,
                props: value.props?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::View> for View {
        fn from(value: super::View) -> Self {
            Self {
                context: Ok(value.context),
                find: Ok(value.find),
                name: Ok(value.name),
                props: Ok(value.props),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ViewDefinitionsFind {
        coll: Result<String, String>,
        projection: Result<Option<super::DataProjection>, String>,
        query: Result<super::DataQuery, String>,
    }
    impl Default for ViewDefinitionsFind {
        fn default() -> Self {
            Self {
                coll: Err("no value supplied for coll".to_string()),
                projection: Ok(Default::default()),
                query: Err("no value supplied for query".to_string()),
            }
        }
    }
    impl ViewDefinitionsFind {
        pub fn coll<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .coll = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coll: {}", e));
            self
        }
        pub fn projection<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DataProjection>>,
            T::Error: std::fmt::Display,
        {
            self
                .projection = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for projection: {}", e)
                });
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DataQuery>,
            T::Error: std::fmt::Display,
        {
            self
                .query = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for query: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<ViewDefinitionsFind> for super::ViewDefinitionsFind {
        type Error = String;
        fn try_from(value: ViewDefinitionsFind) -> Result<Self, String> {
            Ok(Self {
                coll: value.coll?,
                projection: value.projection?,
                query: value.query?,
            })
        }
    }
    impl From<super::ViewDefinitionsFind> for ViewDefinitionsFind {
        fn from(value: super::ViewDefinitionsFind) -> Self {
            Self {
                coll: Ok(value.coll),
                projection: Ok(value.projection),
                query: Ok(value.query),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Wrap {
        alignment: Result<Option<super::StylesWrapAlignment>, String>,
        children: Result<Vec<super::LenraComponent>, String>,
        cross_axis_alignment: Result<Option<super::StylesWrapCrossAlignment>, String>,
        direction: Result<Option<super::StylesDirection>, String>,
        horizontal_direction: Result<Option<super::StylesTextDirection>, String>,
        run_alignment: Result<Option<super::StylesWrapAlignment>, String>,
        run_spacing: Result<Option<f64>, String>,
        spacing: Result<Option<f64>, String>,
        type_: Result<serde_json::Value, String>,
        vertical_direction: Result<Option<super::StylesVerticalDirection>, String>,
    }
    impl Default for Wrap {
        fn default() -> Self {
            Self {
                alignment: Ok(Default::default()),
                children: Err("no value supplied for children".to_string()),
                cross_axis_alignment: Ok(Default::default()),
                direction: Ok(Default::default()),
                horizontal_direction: Ok(Default::default()),
                run_alignment: Ok(Default::default()),
                run_spacing: Ok(Default::default()),
                spacing: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                vertical_direction: Ok(Default::default()),
            }
        }
    }
    impl Wrap {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesWrapAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .alignment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alignment: {}", e)
                });
            self
        }
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LenraComponent>>,
            T::Error: std::fmt::Display,
        {
            self
                .children = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for children: {}", e)
                });
            self
        }
        pub fn cross_axis_alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesWrapCrossAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .cross_axis_alignment = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for cross_axis_alignment: {}", e
                    )
                });
            self
        }
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .direction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for direction: {}", e)
                });
            self
        }
        pub fn horizontal_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesTextDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .horizontal_direction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for horizontal_direction: {}", e
                    )
                });
            self
        }
        pub fn run_alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesWrapAlignment>>,
            T::Error: std::fmt::Display,
        {
            self
                .run_alignment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for run_alignment: {}", e)
                });
            self
        }
        pub fn run_spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .run_spacing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for run_spacing: {}", e)
                });
            self
        }
        pub fn spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self
                .spacing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for spacing: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn vertical_direction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StylesVerticalDirection>>,
            T::Error: std::fmt::Display,
        {
            self
                .vertical_direction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for vertical_direction: {}", e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<Wrap> for super::Wrap {
        type Error = String;
        fn try_from(value: Wrap) -> Result<Self, String> {
            Ok(Self {
                alignment: value.alignment?,
                children: value.children?,
                cross_axis_alignment: value.cross_axis_alignment?,
                direction: value.direction?,
                horizontal_direction: value.horizontal_direction?,
                run_alignment: value.run_alignment?,
                run_spacing: value.run_spacing?,
                spacing: value.spacing?,
                type_: value.type_?,
                vertical_direction: value.vertical_direction?,
            })
        }
    }
    impl From<super::Wrap> for Wrap {
        fn from(value: super::Wrap) -> Self {
            Self {
                alignment: Ok(value.alignment),
                children: Ok(value.children),
                cross_axis_alignment: Ok(value.cross_axis_alignment),
                direction: Ok(value.direction),
                horizontal_direction: Ok(value.horizontal_direction),
                run_alignment: Ok(value.run_alignment),
                run_spacing: Ok(value.run_spacing),
                spacing: Ok(value.spacing),
                type_: Ok(value.type_),
                vertical_direction: Ok(value.vertical_direction),
            }
        }
    }
}


pub fn actionable<T0>(child: T0) -> builder::Actionable 
where
    T0: std::convert::TryInto<LenraComponent>,
    T0::Error: std::fmt::Display,
 {
    Actionable::builder()
        .type_("actionable")
        .child(child)
}

pub fn button<T0>(text: T0) -> builder::Button 
where
    T0: std::convert::TryInto<String>,
    T0::Error: std::fmt::Display,
 {
    Button::builder()
        .type_("button")
        .text(text)
}

pub fn carousel<T0>(children: T0) -> builder::Carousel 
where
    T0: std::convert::TryInto<Vec < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Carousel::builder()
        .type_("carousel")
        .children(children)
}

pub fn checkbox<T0>(value: T0) -> builder::Checkbox 
where
    T0: std::convert::TryInto<bool>,
    T0::Error: std::fmt::Display,
 {
    Checkbox::builder()
        .type_("checkbox")
        .value(value)
}

pub fn container() -> builder::Container {
    Container::builder()
        .type_("container")
}

pub fn dropdownbutton<T0, T1>(child: T0, text: T1) -> builder::DropdownButton 
where
    T0: std::convert::TryInto<Box < LenraComponent >>,
    T0::Error: std::fmt::Display,
    T1: std::convert::TryInto<String>,
    T1::Error: std::fmt::Display,
 {
    DropdownButton::builder()
        .type_("dropdownbutton")
        .child(child)
        .text(text)
}

pub fn flex<T0>(children: T0) -> builder::Flex 
where
    T0: std::convert::TryInto<Vec < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Flex::builder()
        .type_("flex")
        .children(children)
}

pub fn flexible<T0>(child: T0) -> builder::Flexible 
where
    T0: std::convert::TryInto<Box < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Flexible::builder()
        .type_("flexible")
        .child(child)
}

pub fn form<T0>(child: T0) -> builder::Form 
where
    T0: std::convert::TryInto<Box < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Form::builder()
        .type_("form")
        .child(child)
}

pub fn icon<T0>(value: T0) -> builder::Icon 
where
    T0: std::convert::TryInto<StylesIconName>,
    T0::Error: std::fmt::Display,
 {
    Icon::builder()
        .type_("icon")
        .value(value)
}

pub fn image<T0>(src: T0) -> builder::Image 
where
    T0: std::convert::TryInto<String>,
    T0::Error: std::fmt::Display,
 {
    Image::builder()
        .type_("image")
        .src(src)
}

pub fn menu<T0>(children: T0) -> builder::Menu 
where
    T0: std::convert::TryInto<Vec < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Menu::builder()
        .type_("menu")
        .children(children)
}

pub fn menuitem<T0>(text: T0) -> builder::MenuItem 
where
    T0: std::convert::TryInto<String>,
    T0::Error: std::fmt::Display,
 {
    MenuItem::builder()
        .type_("menuitem")
        .text(text)
}

pub fn overlayentry<T0>(child: T0) -> builder::OverlayEntry 
where
    T0: std::convert::TryInto<Box < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    OverlayEntry::builder()
        .type_("overlayentry")
        .child(child)
}

pub fn radio<T0, T1>(group_value: T0, value: T1) -> builder::Radio 
where
    T0: std::convert::TryInto<String>,
    T0::Error: std::fmt::Display,
    T1: std::convert::TryInto<String>,
    T1::Error: std::fmt::Display,
 {
    Radio::builder()
        .type_("radio")
        .group_value(group_value)
        .value(value)
}

pub fn slider() -> builder::Slider {
    Slider::builder()
        .type_("slider")
}

pub fn stack<T0>(children: T0) -> builder::Stack 
where
    T0: std::convert::TryInto<Vec < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Stack::builder()
        .type_("stack")
        .children(children)
}

pub fn statussticker<T0>(status: T0) -> builder::StatusSticker 
where
    T0: std::convert::TryInto<StatusStickerStatus>,
    T0::Error: std::fmt::Display,
 {
    StatusSticker::builder()
        .type_("statussticker")
        .status(status)
}

pub fn text<T0>(value: T0) -> builder::Text 
where
    T0: std::convert::TryInto<String>,
    T0::Error: std::fmt::Display,
 {
    Text::builder()
        .type_("text")
        .value(value)
}

pub fn toggle<T0>(value: T0) -> builder::Toggle 
where
    T0: std::convert::TryInto<bool>,
    T0::Error: std::fmt::Display,
 {
    Toggle::builder()
        .type_("toggle")
        .value(value)
}

pub fn view<T0>(name: T0) -> builder::View 
where
    T0: std::convert::TryInto<String>,
    T0::Error: std::fmt::Display,
 {
    View::builder()
        .type_("view")
        .name(name)
}

pub fn wrap<T0>(children: T0) -> builder::Wrap 
where
    T0: std::convert::TryInto<Vec < LenraComponent >>,
    T0::Error: std::fmt::Display,
 {
    Wrap::builder()
        .type_("wrap")
        .children(children)
}