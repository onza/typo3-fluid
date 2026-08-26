/**
 * Fluid tree-sitter grammar (HTML + Fluid expressions).
 * Scanner allows '.'/'_' in tag names so <f:format.raw> is one tag.
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'fluid',

  word: $ => $.identifier,

  extras: $ => [
    $.comment,
    /\s+/,
  ],

  externals: $ => [
    $._start_tag_name,
    $._script_start_tag_name,
    $._style_start_tag_name,
    $._end_tag_name,
    $.erroneous_end_tag_name,
    '/>',
    $._implicit_end_tag,
    $.raw_text,
    $.comment,
  ],

  rules: {
    document: $ => repeat($._node),

    doctype: $ => seq('<!', alias($._doctype, 'doctype'), /[^>]+/, '>'),
    _doctype: _ => /[Dd][Oo][Cc][Tt][Yy][Pp][Ee]/,

    _node: $ => choice(
      $.fluid_comment,
      $.doctype,
      $.cdata,
      $.entity,
      $.expression,
      $.text,
      $.element,
      $.script_element,
      $.style_element,
      $.erroneous_end_tag,
    ),

    // {* ... *} — HTML <!-- --> is external $.comment
    fluid_comment: _ => token(seq('{*', /([^*]|\*+[^*}])*/, '*}')),

    // CDATA content not tokenized
    cdata: _ => token(seq(
      '<![CDATA[',
      repeat(choice(/[^\]]/, /\][^\]]/, /\]\][^>]/)),
      ']]>',
    )),

    element: $ => choice(
      seq($.start_tag, repeat($._node), choice($.end_tag, $._implicit_end_tag)),
      // <{headline}>…</{headline}>
      seq($.dynamic_start_tag, repeat($._node), $.dynamic_end_tag),
      $.self_closing_tag,
    ),

    dynamic_start_tag: $ => seq('<', $.expression, repeat($._tag_part), '>'),
    dynamic_end_tag: $ => seq('</', $.expression, '>'),

    script_element: $ => seq(alias($.script_start_tag, $.start_tag), optional($.raw_text), $.end_tag),
    style_element: $ => seq(alias($.style_start_tag, $.start_tag), optional($.raw_text), $.end_tag),

    start_tag: $ => seq('<', alias($._start_tag_name, $.tag_name), repeat($._tag_part), '>'),
    script_start_tag: $ => seq('<', alias($._script_start_tag_name, $.tag_name), repeat($._tag_part), '>'),
    style_start_tag: $ => seq('<', alias($._style_start_tag_name, $.tag_name), repeat($._tag_part), '>'),
    self_closing_tag: $ => seq('<', alias($._start_tag_name, $.tag_name), repeat($._tag_part), '/>'),

    // Bare expressions as tag parts: <a {f:if(...)} {_all}>
    _tag_part: $ => choice($.attribute, $.expression),

    end_tag: $ => seq('</', alias($._end_tag_name, $.tag_name), '>'),
    erroneous_end_tag: $ => seq('</', $.erroneous_end_tag_name, '>'),

    attribute: $ => seq(
      $.attribute_name,
      optional(seq('=', choice($.attribute_value, $.quoted_attribute_value))),
    ),

    attribute_name: _ => /[^<>"'/=\s{}]+/,
    attribute_value: _ => /[^<>"'=\s{}]+/,

    entity: _ => /&(#([xX][0-9a-fA-F]{1,6}|[0-9]{1,5})|[A-Za-z]{1,30});?/,

    // Quoted values may mix literals and {expressions} (and \ escapes)
    quoted_attribute_value: $ => choice(
      seq("'", repeat(choice($.expression, alias(token(/([^'{\\]|\\.)+/), $.attribute_value))), "'"),
      seq('"', repeat(choice($.expression, alias(token(/([^"{\\]|\\.)+/), $.attribute_value))), '"'),
    ),

    text: _ => token(prec(-1, /[^<>&{}\s]([^<>&{}]*[^<>&{}\s])?/)),

    expression: $ => seq('{', optional($._expr_inner), '}'),

    _expr_inner: $ => choice(
      $.namespace_definition,
      $.array,
      $._compound,
    ),

    namespace_definition: $ => seq(
      'namespace',
      $.namespace,
      optional(seq('=', $.php_class)),
    ),
    namespace: _ => /[A-Za-z_*][A-Za-z0-9_.*]*/,
    php_class: _ => /[A-Za-z0-9_]+(\\[A-Za-z0-9_]+)+/,

    array: $ => seq($.pair, repeat(seq(',', $.pair)), optional(',')),
    pair: $ => seq(field('key', $._key), ':', field('value', $._value)),
    _key: $ => choice(alias($.identifier, $.array_key), $.string, $.number),

    _compound: $ => prec.right(seq(
      $._value,
      repeat(choice($._operation, $.pipe, $.ternary, $.cast)),
    )),

    _operation: $ => seq($.operator, $._value),
    pipe: $ => seq('->', $.inline_viewhelper),
    ternary: $ => choice(
      seq('?', $._value, ':', $._value),
      seq('?:', $._value),
    ),
    cast: $ => seq('as', alias($.identifier, $.type)),

    _value: $ => choice(
      $.unary,
      $.inline_viewhelper,
      $.boolean,
      $.null,
      $.special_variable,
      $.number,
      $.string,
      $.expression,
      $.variable,
    ),

    unary: $ => prec(3, seq($.operator, $._value)),

    inline_viewhelper: $ => seq(
      field('name', $.viewhelper_name),
      '(',
      optional($.arguments),
      ')',
    ),
    viewhelper_name: _ => token(/[A-Za-z_][A-Za-z0-9_]*:[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]*)*/),
    arguments: $ => seq($.argument, repeat(seq(',', $.argument)), optional(',')),
    argument: $ => seq(field('name', alias($.identifier, $.argument_name)), ':', field('value', $._value)),

    boolean: _ => choice('true', 'false'),
    null: _ => 'null',
    special_variable: _ => '_all',
    number: _ => token(/-?\d+(\.\d+)?/),
    // {arr.0} → number nodes for numeric path segments
    variable: $ => prec.left(seq($.identifier, repeat(seq('.', choice($.identifier, alias(token(/\d+/), $.number)))))),
    identifier: _ => /[A-Za-z_][A-Za-z0-9_]*/,

    // No interpolation inside strings
    string: _ => token(choice(
      seq("'", repeat(choice(/\\./, /[^'\\]/)), "'"),
      seq('"', repeat(choice(/\\./, /[^"\\]/)), '"'),
    )),

    operator: _ => token(choice(
      '+', '-', '*', '/', '%', '^', '!',
      '==', '===', '!=', '!==', '>=', '<=', '>', '<',
      '&&', '||',
    )),
  },
});
