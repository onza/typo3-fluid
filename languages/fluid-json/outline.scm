(element
  (start_tag
    (tag_name) @context
    (attribute
      (attribute_name) @_n
      (quoted_attribute_value (attribute_value) @name))
    (#match? @context ":section$")
    (#eq? @_n "name"))) @item

(self_closing_tag
  (tag_name) @context
  (attribute
    (attribute_name) @_n
    (quoted_attribute_value (attribute_value) @name))
  (#match? @context ":(layout|render|section)$")
  (#match? @_n "^(name|section|partial)$")) @item
