//! Модель XML-документа, выгружаемого из GX-Works 3

use serde::{self, Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Document {
    #[serde(rename = "fileHeader")]
    pub file_header: FileHeader,
    #[serde(rename = "contentHeader")]
    pub content_header: StringHeader,
    pub instances: Instances,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct FileHeader {
    #[serde(rename = "@companyName")]
    pub company_name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct StringHeader {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Instances {
    pub configurations: Configurations,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Configurations {
    pub configuration: Configuration,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Configuration {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "globalVars")]
    pub global_vars: Vec<GlobalVars>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct GlobalVars {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub variable: Vec<Variable>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Variable {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@address")]
    pub address: Option<String>,
    #[serde(rename = "type")]
    pub type_: VariableType,
    #[serde(rename = "addData")]
    pub add_data: VariableAddData,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VariableType {
    #[serde(rename = "$value")]
    pub content: VariableTypes,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub enum VariableTypes {
    #[serde(rename = "array")]
    Array(Array),
    #[serde(rename = "BOOL")]
    Bool,
    #[serde(rename = "derived")]
    Derived,
    #[serde(rename = "DINT")]
    Dint,
    #[serde(rename = "DWORD")]
    Dword,
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "WORD")]
    Word,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VariableAddData {
    #[serde(default)]
    pub data: Vec<VariableAddDataItem>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VariableAddDataItem {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub content: VariableAddDataItemContent,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum VariableAddDataItemContent {
    #[serde(rename = "variableLineNumber")]
    VariableLineNumber,
    #[serde(rename = "variableComments")]
    VariableComments,
    #[serde(rename = "variableStructDeviceAssignment")]
    VariableStructDeviceAssignment(VariableStructDeviceAssignment),
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VariableStructDeviceAssignment {
    #[serde(rename = "$value")]
    pub content: Vec<VariableStructDeviceAssignmentItem>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum VariableStructDeviceAssignmentItem {
    #[serde(rename = "array")]
    Array(VariableStructDeviceAssignmentItemArray),
    #[serde(rename = "member")]
    Member(MemberInStruct),
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct MemberInStruct {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@address")]
    address: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VariableStructDeviceAssignmentItemArray {
    #[serde(rename = "@wordAddress")]
    pub word_address: String,
    pub element: Vec<VariableStructDeviceAssignmentItemArrayElement>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VariableStructDeviceAssignmentItemArrayElement {
    #[serde(rename = "@index")]
    pub index: i32,
    pub member: Vec<MemberInStruct>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Array {
    pub dimension: ArrayDimension,
    #[serde(rename = "baseType")]
    pub base_type: ArrayBaseType,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ArrayDimension {
    #[serde(rename = "@lower")]
    lower: i32,
    #[serde(rename = "@upper")]
    upper: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ArrayBaseType {
    #[serde(rename = "$value")]
    data_type: ArrayBaseTypes,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
enum ArrayBaseTypes {
    #[serde(rename = "BOOL")]
    Bool,
    #[serde(rename = "derived")]
    Derived,
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "REAL")]
    Real,
}
