//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttTargetEvalEvaluate {
    /**If an element pointed to is itself a pointer, then the target of that pointer will
              be taken, and so on, until an element is found which is not a pointer.*/
    #[serde(rename = "all")]
    All,
    /**If an element pointed to is itself a pointer, then its target (whether a pointer
              or not) is taken as the target of this pointer.*/
    #[serde(rename = "one")]
    One,
    /**No further evaluation of targets is carried out beyond that needed to find the
              element(s) specified in plist or target attribute.*/
    #[serde(rename = "none")]
    None,
}
///Attributes that deal with resolution of values in plist or target attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTargetEval {
    /**Specifies the intended meaning when a participant in a relationship is itself a
          pointer.*/
    #[serde(rename = "@evaluate", skip_serializing_if = "Option::is_none")]
    pub evaluate: Option<AttTargetEvalEvaluate>,
}
