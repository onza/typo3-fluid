(fluid_comment) @comment
(comment) @comment

(tag_name) @tag
(doctype) @tag.doctype
(attribute_name) @attribute
(attribute_value) @string
(cdata) @string
(entity) @string.special

"=" @punctuation.delimiter.html

[
  "<"
  ">"
  "<!"
  "</"
  "/>"
] @punctuation.bracket.html

; ViewHelper tags (later patterns win): <f:if>, <v:…>
((tag_name) @function
  (#match? @function "^[A-Za-z_][A-Za-z0-9_*]*(\\.[A-Za-z0-9_]+)*:[A-Za-z0-9_.]+$"))

((attribute_name) @keyword
  (#match? @keyword "^(xmlns:[A-Za-z]|data-namespace-typo3-fluid)"))

(expression ["{" "}"] @punctuation.special)

(variable (identifier) @variable)
(variable "." @punctuation.delimiter)

(viewhelper_name) @function
(argument_name) @property
(array_key) @property
(type) @type
(boolean) @constant.builtin
(null) @constant.builtin
(special_variable) @variable.builtin
(number) @number
(string) @string
(operator) @operator

(namespace_definition "namespace" @keyword)
(namespace) @variable
(php_class) @type
(cast "as" @keyword)
(pipe "->" @operator)
(ternary ["?" ":" "?:"] @operator)
(pair ":" @punctuation.delimiter)
(argument ":" @punctuation.delimiter)

[
  ","
  "("
  ")"
] @punctuation.bracket

; <f:comment> body is never rendered
((element (start_tag (tag_name) @_n) (text) @comment)
  (#eq? @_n "f:comment"))
