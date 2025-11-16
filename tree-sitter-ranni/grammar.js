/**
 * @file Meta-reductive programming language
 * @author Steven vanZyl <rushsteve1@rushsteve1.us>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "ranni",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
