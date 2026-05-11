use std::{
    fmt,
    str::{self, FromStr},
    time::Duration,
};

use crate::{
    gdtf::{Name, Node, NodePath, bundle},
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DmxMode {
    name: Name,
    description: Option<String>,
    geometry: NodePath,

    dmx_channels: Vec<DmxChannel>,
    relations: Vec<Relation>,
    ft_macros: Vec<FtMacro>,
}

impl DmxMode {
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn geometry(&self) -> &NodePath {
        &self.geometry
    }

    pub fn dmx_channels(&self) -> &[DmxChannel] {
        &self.dmx_channels
    }

    pub fn relations(&self) -> &[Relation] {
        &self.relations
    }

    pub fn ft_macros(&self) -> &[FtMacro] {
        &self.ft_macros
    }
}

impl Node for DmxMode {
    fn name(&self) -> Option<Name> {
        Some(self.name.clone())
    }
}

impl bundle::FromBundle for DmxMode {
    type Source = bundle::DmxMode;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(&source.name),
            description: match source.description.as_deref() {
                Some("") | None => None,
                Some(s) => Some(s.to_string()),
            },
            geometry: match &source.geometry {
                Some(name) => NodePath::from_str(name).unwrap(),
                None => {
                    let first_geometry_name = bundle
                        .description()
                        .fixture_type
                        .geometries
                        .children
                        .first()
                        .expect("FIXME: Find out what to do in this case")
                        .name();
                    NodePath::from_str(first_geometry_name).unwrap()
                }
            },
            dmx_channels: source
                .dmx_channels
                .dmx_channels
                .iter()
                .map(|dc| DmxChannel::from_bundle(dc, bundle))
                .collect(),
            relations: source
                .relations
                .iter()
                .flat_map(|r| r.relations.iter().map(|r| Relation::from_bundle(r, bundle)))
                .collect(),
            ft_macros: source
                .ft_macros
                .iter()
                .flat_map(|ftm| ftm.ft_macros.iter().map(|ftm| FtMacro::from_bundle(ftm, bundle)))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxChannel {
    dmx_break: DmxBreak,
    offset: DmxOffset,
    initial_function: Option<NodePath>,
    highlight: Option<DmxValue>,
    geometry: NodePath,

    logical_channels: Vec<LogicalChannel>,
}

impl DmxChannel {
    pub fn dmx_break(&self) -> DmxBreak {
        self.dmx_break
    }

    pub fn offset(&self) -> &DmxOffset {
        &self.offset
    }

    pub fn initial_function(&self) -> &NodePath {
        match &self.initial_function {
            Some(cf) => cf,
            None => todo!("get the first LC's first CF "),
        }
    }

    pub fn highlight(&self) -> Option<DmxValue> {
        self.highlight
    }

    pub fn geometry(&self) -> &NodePath {
        &self.geometry
    }

    pub fn logical_channels(&self) -> &[LogicalChannel] {
        &self.logical_channels
    }
}

impl bundle::FromBundle for DmxChannel {
    type Source = bundle::DmxChannel;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            dmx_break: DmxBreak::from_str(&source.dmx_break).unwrap(),
            offset: DmxOffset::from_str(&source.offset).unwrap(),
            initial_function: source
                .initial_function
                .as_ref()
                .map(|node| NodePath::from_str(node).unwrap()),
            highlight: if source.highlight.trim() == "None" {
                None
            } else {
                Some(DmxValue::from_str(&source.highlight).unwrap())
            },
            geometry: NodePath::from_str(&source.geometry).unwrap(),
            logical_channels: source
                .logical_channels
                .iter()
                .map(|lc| LogicalChannel::from_bundle(lc, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DmxBreak {
    Overwrite,
    Break(u8),
}

impl str::FromStr for DmxBreak {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Overwrite" {
            Ok(DmxBreak::Overwrite)
        } else {
            match s.trim().parse::<u32>() {
                Ok(n) if n > 0 => Ok(DmxBreak::Break(n as u8)),
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalChannel {
    attribute: NodePath,
    snap: Snap,
    master: Master,
    mib_fade: Duration,
    dmx_change_time_limit: Duration,

    channel_function: Vec<ChannelFunction>,
}

impl LogicalChannel {
    pub fn attribute(&self) -> &NodePath {
        &self.attribute
    }

    pub fn snap(&self) -> Snap {
        self.snap
    }

    pub fn master(&self) -> Master {
        self.master
    }

    pub fn mib_fade(&self) -> Duration {
        self.mib_fade
    }

    pub fn dmx_change_time_limit(&self) -> Duration {
        self.dmx_change_time_limit
    }

    pub fn channel_function(&self) -> &[ChannelFunction] {
        &self.channel_function
    }
}

impl bundle::FromBundle for LogicalChannel {
    type Source = bundle::LogicalChannel;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            attribute: NodePath::from_str(&source.attribute).unwrap(),
            snap: Snap::from_bundle(&source.snap, bundle),
            master: Master::from_bundle(&source.master, bundle),
            mib_fade: util::parse_possibly_negative_duration(source.mib_fade.unwrap_or(0.0)),
            dmx_change_time_limit: util::parse_possibly_negative_duration(
                source.dmx_change_time_limit.unwrap_or(0.0),
            ),
            channel_function: source
                .channel_functions
                .iter()
                .map(|cf| ChannelFunction::from_bundle(cf, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Snap {
    #[default]
    No,
    Yes,
    Off,
    On,
}

impl bundle::FromBundle for Snap {
    type Source = bundle::Snap;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::Snap::Yes => Snap::Yes,
            bundle::Snap::No => Snap::No,
            bundle::Snap::On => Snap::On,
            bundle::Snap::Off => Snap::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Master {
    #[default]
    None,
    Grand,
    Group,
}

impl bundle::FromBundle for Master {
    type Source = bundle::Master;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::Master::None => Master::None,
            bundle::Master::Grand => Master::Grand,
            bundle::Master::Group => Master::Group,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelFunction {
    name: Option<Name>,
    attribute: NodePath,
    original_attribute: Option<String>,
    dmx_from: DmxValue,
    default: DmxValue,
    physical_from: f32,
    physical_to: f32,
    real_fade: Duration,
    real_acceleration: f32,
    wheel: Option<NodePath>,
    emitter: Option<NodePath>,
    filter: Option<NodePath>,
    color_space: Option<NodePath>,
    gamut: Option<NodePath>,
    mode_master: Option<ModeMaster>,
    dmx_profile: Option<NodePath>,
    min: f32,
    max: f32,
    custom_name: Option<String>,

    channel_sets: Vec<ChannelSet>,
    sub_channel_sets: Vec<SubChannelSet>,
}

impl ChannelFunction {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn attribute(&self) -> &NodePath {
        &self.attribute
    }

    pub fn original_attribute(&self) -> Option<&String> {
        self.original_attribute.as_ref()
    }

    pub fn dmx_from(&self) -> DmxValue {
        self.dmx_from
    }

    pub fn default(&self) -> DmxValue {
        self.default
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }

    pub fn real_fade(&self) -> Duration {
        self.real_fade
    }

    pub fn real_acceleration(&self) -> f32 {
        self.real_acceleration
    }

    pub fn wheel(&self) -> Option<&NodePath> {
        self.wheel.as_ref()
    }

    pub fn emitter(&self) -> Option<&NodePath> {
        self.emitter.as_ref()
    }

    pub fn filter(&self) -> Option<&NodePath> {
        self.filter.as_ref()
    }

    pub fn color_space(&self) -> Option<&NodePath> {
        self.color_space.as_ref()
    }

    pub fn gamut(&self) -> Option<&NodePath> {
        self.gamut.as_ref()
    }

    pub fn mode_master(&self) -> Option<&ModeMaster> {
        self.mode_master.as_ref()
    }

    pub fn dmx_profile(&self) -> Option<&NodePath> {
        self.dmx_profile.as_ref()
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn channel_sets(&self) -> &[ChannelSet] {
        &self.channel_sets
    }

    pub fn sub_channel_sets(&self) -> &[SubChannelSet] {
        &self.sub_channel_sets
    }
}

impl bundle::FromBundle for ChannelFunction {
    type Source = bundle::ChannelFunction;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: source.name.as_ref().map(Name::new),
            attribute: NodePath::from_str(&source.attribute).unwrap(),
            original_attribute: match source.original_attribute.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
            dmx_from: DmxValue::from_str(&source.dmx_from).unwrap(),
            default: DmxValue::from_str(&source.dmx_from).unwrap(),
            physical_from: source.physical_from.unwrap_or(0.0),
            physical_to: source.physical_to.unwrap_or(1.0),
            real_fade: util::parse_possibly_negative_duration(source.real_fade.unwrap_or(0.0)),
            real_acceleration: source.real_acceleration.unwrap_or(0.0),
            wheel: source.wheel.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            emitter: source.emitter.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            filter: source.filter.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            color_space: source.color_space.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            gamut: source.gamut.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            mode_master: source.mode_master.as_ref().map(|mm| ModeMaster {
                node: NodePath::from_str(mm).unwrap(),
                from: DmxValue::from_str(&source.mode_from).unwrap(),
                to: DmxValue::from_str(&source.mode_to).unwrap(),
            }),
            dmx_profile: source.dmx_profile.as_ref().map(|s| NodePath::from_str(s).unwrap()),
            min: source.min.unwrap_or(source.physical_from.unwrap_or(0.0)),
            max: source.max.unwrap_or(source.physical_to.unwrap_or(1.0)),
            custom_name: source.custom_name.clone(),
            channel_sets: source
                .channel_sets
                .iter()
                .map(|cs| ChannelSet::from_bundle(cs, bundle))
                .collect(),
            sub_channel_sets: source
                .sub_channel_sets
                .iter()
                .map(|scs| SubChannelSet::from_bundle(scs, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModeMaster {
    node: NodePath,
    from: DmxValue,
    to: DmxValue,
}

impl ModeMaster {
    pub fn node(&self) -> &NodePath {
        &self.node
    }

    pub fn from(&self) -> DmxValue {
        self.from
    }

    pub fn to(&self) -> DmxValue {
        self.to
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelSet {
    name: Option<Name>,
    dmx_from: DmxValue,
    physical_from: f32,
    physical_to: f32,
    wheel_slot_index: Option<u32>,
}

impl ChannelSet {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn dmx_from(&self) -> DmxValue {
        self.dmx_from
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }

    pub fn wheel_slot_index(&self) -> Option<u32> {
        self.wheel_slot_index
    }
}

impl bundle::FromBundle for ChannelSet {
    type Source = bundle::ChannelSet;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            name: source.name.as_ref().map(Name::new),
            dmx_from: DmxValue::from_str(&source.dmx_from).unwrap(),
            physical_from: source.physical_from.unwrap_or(0.0),
            physical_to: source.physical_to.unwrap_or(1.0),
            wheel_slot_index: source.wheel_slot_index.map(|i| i as u32),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubChannelSet {
    name: Option<Name>,
    physical_from: f32,
    physical_to: f32,
    sub_physical_unit: NodePath,
    dmx_profile: Option<NodePath>,
}

impl SubChannelSet {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }

    pub fn sub_physical_unit(&self) -> &NodePath {
        &self.sub_physical_unit
    }

    pub fn dmx_profile(&self) -> Option<&NodePath> {
        self.dmx_profile.as_ref()
    }
}

impl bundle::FromBundle for SubChannelSet {
    type Source = bundle::SubChannelSet;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            name: source.name.as_ref().map(Name::new),
            physical_from: source.physical_from,
            physical_to: source.physical_to,
            sub_physical_unit: NodePath::from_str(&source.sub_physical_unit).unwrap(),
            dmx_profile: source.dmx_profile.as_ref().map(|s| NodePath::from_str(s).unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    name: Name,
    master: NodePath,
    follower: NodePath,
    kind: RelationKind,
}

impl Relation {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn master(&self) -> &NodePath {
        &self.master
    }

    pub fn follower(&self) -> &NodePath {
        &self.follower
    }

    pub fn kind(&self) -> RelationKind {
        self.kind
    }
}

impl bundle::FromBundle for Relation {
    type Source = bundle::Relation;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(&source.name),
            master: NodePath::from_str(&source.master).unwrap(),
            follower: NodePath::from_str(&source.follower).unwrap(),
            kind: RelationKind::from_bundle(&source.r#type, bundle),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationKind {
    Multiply,
    Override,
}

impl bundle::FromBundle for RelationKind {
    type Source = bundle::RelationType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::RelationType::Multiply => RelationKind::Multiply,
            bundle::RelationType::Override => RelationKind::Override,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FtMacro {
    name: Name,
    channel_function: Option<NodePath>,
    dmx: Vec<MacroDmx>,
}

impl FtMacro {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn channel_function(&self) -> Option<&NodePath> {
        self.channel_function.as_ref()
    }

    pub fn dmx(&self) -> &[MacroDmx] {
        &self.dmx
    }
}

impl bundle::FromBundle for FtMacro {
    type Source = bundle::FtMacro;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(&source.name),
            channel_function: source
                .channel_function
                .as_ref()
                .map(|s| NodePath::from_str(s).unwrap()),
            dmx: source.macro_dmx.iter().map(|v| MacroDmx::from_bundle(v, bundle)).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDmx {
    steps: Vec<MacroDmxStep>,
}

impl MacroDmx {
    pub fn steps(&self) -> &[MacroDmxStep] {
        &self.steps
    }
}

impl bundle::FromBundle for MacroDmx {
    type Source = bundle::MacroDmx;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            steps: source
                .macro_dmx_steps
                .iter()
                .map(|s| MacroDmxStep::from_bundle(s, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDmxStep {
    duration: Duration,
    values: Vec<MacroDmxValue>,
}

impl MacroDmxStep {
    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn values(&self) -> &[MacroDmxValue] {
        &self.values
    }
}

impl bundle::FromBundle for MacroDmxStep {
    type Source = bundle::MacroDmxStep;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            duration: util::parse_possibly_negative_duration(source.duration.unwrap_or_default()),
            values: source
                .macro_dmx_values
                .iter()
                .map(|v| MacroDmxValue::from_bundle(v, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDmxValue {
    value: DmxValue,
    dmx_channel: NodePath,
}

impl MacroDmxValue {
    pub fn value(&self) -> DmxValue {
        self.value
    }

    pub fn dmx_channel(&self) -> &NodePath {
        &self.dmx_channel
    }
}

impl bundle::FromBundle for MacroDmxValue {
    type Source = bundle::MacroDmxValue;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            value: DmxValue::from_str(&source.value).unwrap(),
            dmx_channel: NodePath::from_str(&source.dmx_channel).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmxValue {
    value: u32,
    bytes: u8,
    shifting: bool,
}

impl DmxValue {
    pub fn from_u8(value: u8, shifting: bool) -> Self {
        DmxValue { value: value as u32, bytes: 1, shifting }
    }

    pub fn from_u16(value: u16, shifting: bool) -> Self {
        DmxValue { value: value as u32, bytes: 2, shifting }
    }

    pub fn from_u24(value: u32, shifting: bool) -> Self {
        if value > 0xFFFFFF {
            todo!();
        }
        DmxValue { value, bytes: 3, shifting }
    }

    pub fn from_u32(value: u32, shifting: bool) -> Self {
        DmxValue { value, bytes: 4, shifting }
    }
}

impl Default for DmxValue {
    fn default() -> Self {
        Self { value: 0, bytes: 1, shifting: false }
    }
}

impl str::FromStr for DmxValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (value_s, rest) = s.split_once('/').ok_or_else(|| {
            todo!();
        })?;

        let value = match value_s.trim().parse::<u32>() {
            Ok(v) => v,
            Err(_) => match value_s.trim().parse::<i64>() {
                Ok(v) => {
                    if v < 0 {
                        eprintln!("DmxValue contained negative value. Clamping.");
                        0
                    } else if v > u32::MAX as i64 {
                        eprintln!("DmxValue exceeded u32::MAX. Clamping.");
                        u32::MAX
                    } else {
                        v as u32
                    }
                }
                Err(_) => todo!(),
            },
        };

        let rest = rest.trim();
        let (bytes_s, shifting) = if let Some(bytes_s) = rest.strip_suffix('s') {
            (bytes_s, true)
        } else {
            (rest, false)
        };

        let bytes_u32: u32 = bytes_s.parse().map_err(|_| {
            todo!();
        })?;
        if bytes_u32 == 0 || bytes_u32 > u8::MAX as u32 {
            todo!();
        }

        Ok(DmxValue { value, bytes: bytes_u32 as u8, shifting })
    }
}

impl fmt::Display for DmxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.shifting {
            write!(f, "{}/{}s", self.value, self.bytes)
        } else {
            write!(f, "{}/{}", self.value, self.bytes)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmxOffset {
    Offsets(Vec<u32>),
    Virtual,
}

impl str::FromStr for DmxOffset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() || s == "None" {
            return Ok(DmxOffset::Virtual);
        }

        let mut values = Vec::new();
        for part in s.split(',') {
            if part.is_empty() {
                todo!();
            }
            let v = part.trim().parse().unwrap();
            values.push(v);
        }

        Ok(DmxOffset::Offsets(values))
    }
}
