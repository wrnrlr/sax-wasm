use sax::tag::*;

pub fn attribute_to_json(attr: &Attribute) -> String {
  // {"name":"myName", "value":"myValue"}
  let mut attr_json = "{\"name\":\"".to_string();
  attr_json.push_str(attr.name.as_ref());
  attr_json.push_str("\",\"value\":\"");
  attr_json.push_str(attr.value.as_ref());
  attr_json.push_str("\",\"start\":");
  attr_json.push_str(point_to_json(&attr.start).as_ref());
  attr_json.push_str(",\"end\":");
  attr_json.push_str(point_to_json(&attr.end).as_ref());
  attr_json.push_str("}");
  attr_json
}

pub fn attributes_to_json(attrs: &Vec<Attribute>) -> String {
  let mut attrs_json = "[".to_string();
  let len = attrs_json.len();
  let mut i = 0;
  for attr in attrs {
    i += 1;
    attrs_json.push_str(attribute_to_json(attr).as_ref());
    if i != len {
      attrs_json.push_str(",");
    }
  }
  attrs_json.push_str("]");
  attrs_json
}

pub fn point_to_json(pt: &(u32, u32)) -> String {
  let mut pt_json = "{\"line\":".to_string();
  pt_json.push_str(pt.0.to_string().as_ref());
  pt_json.push_str(",\"character\":");
  pt_json.push_str(pt.1.to_string().as_ref());
  pt_json.push_str("}");

  pt_json
}

pub fn tag_to_json(tag: &Tag) -> String {
  // Name
  let mut tag_json = "{\"name\":\"".to_string();
  tag_json.push_str(tag.name.as_ref());
  tag_json.push_str("\",");
  // attributes
  tag_json.push_str("\"attributes\":");
  tag_json.push_str(attributes_to_json(&tag.attributes).as_ref());
  // start, end
  tag_json.push_str(",\"start\":");
  tag_json.push_str(point_to_json(&tag.start).as_ref());
  tag_json.push_str(",\"end\":");
  tag_json.push_str(point_to_json(&tag.end).as_ref());
  tag_json.push_str(",\"text\":\"");
  tag_json.push_str(tag.text.as_ref());
  tag_json.push_str("\"");
  tag_json.push_str(",\"selfClosing\":");
  tag_json.push_str(if tag.self_closing { "true" } else { "false" });
  tag_json.push_str("}");

  tag_json
}
