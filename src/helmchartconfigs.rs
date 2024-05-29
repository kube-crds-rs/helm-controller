// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f helmchartconfigs.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "helm.cattle.io",
    version = "v1",
    kind = "HelmChartConfig",
    plural = "helmchartconfigs"
)]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct HelmChartConfigSpec {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failurePolicy"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub failure_policy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valuesContent"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values_content: Option<String>,
}
