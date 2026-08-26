((comment) @injection.content
  (#set! injection.language "comment"))

(script_element
  (raw_text) @injection.content
  (#set! injection.language "javascript"))

(style_element
  (raw_text) @injection.content
  (#set! injection.language "css"))

; style="…" with {expressions}: combine literal fragments as one CSS doc
(attribute
  (attribute_name) @_attribute_name
  (#eq? @_attribute_name "style")
  (quoted_attribute_value
    (attribute_value) @injection.content)
  (#set! injection.language "css")
  (#set! injection.combined))
