# Files of upstream's code trees

Referenced by: [`MASTER.md`](MASTER.md), which maps these trees by directory. This index lists every
tracked file under `docx-core/` and `docx-wasm/`, under its directory. It is written by
[`scripts/files-index.sh`](scripts/files-index.sh) from `git ls-files`; never edit it by hand. Run
`sh scripts/files-index.sh` after an upstream merge or after adding or removing a file there, and
`sh scripts/files-index.sh --check` to see whether it is current.

The fixtures, snapshots, test output and the built demo page are data, mapped as folders in
`MASTER.md`, and are not listed here.

## `docx-core/`

The crate's manifest, and a reader test upstream keeps at the crate root, outside tests/.

- `docx-core/Cargo.toml`
- `docx-core/test_xml_reader.rs`

## `docx-core/benches/`

Criterion benchmarks: reading and writing a document.

- `docx-core/benches/read_docx.rs`
- `docx-core/benches/write_docx.rs`

## `docx-core/bindings/`

TypeScript types ts-rs generates from the theme structs.

- `docx-core/bindings/FontGroup.ts`
- `docx-core/bindings/FontSchemeFont.ts`
- `docx-core/bindings/FontScheme.ts`
- `docx-core/bindings/Theme.ts`

## `docx-core/examples/`

Runnable examples, one per feature; `cargo run --example <name>` writes into output/examples/.

- `docx-core/examples/alignment.rs`
- `docx-core/examples/bookmark.rs`
- `docx-core/examples/comment.rs`
- `docx-core/examples/custom_property.rs`
- `docx-core/examples/custom_xml.rs`
- `docx-core/examples/data_binding.rs`
- `docx-core/examples/dirty_toc.rs`
- `docx-core/examples/doc_id.rs`
- `docx-core/examples/even_header.rs`
- `docx-core/examples/first_header.rs`
- `docx-core/examples/font.rs`
- `docx-core/examples/font_size.rs`
- `docx-core/examples/footer.rs`
- `docx-core/examples/footnotes.rs`
- `docx-core/examples/header.rs`
- `docx-core/examples/header_with_page_num.rs`
- `docx-core/examples/hello.rs`
- `docx-core/examples/history.rs`
- `docx-core/examples/hyperlink.rs`
- `docx-core/examples/image_floating.rs`
- `docx-core/examples/image_in_header.rs`
- `docx-core/examples/image_inline_rotate.rs`
- `docx-core/examples/image_inline.rs`
- `docx-core/examples/image_reader.rs`
- `docx-core/examples/indent.rs`
- `docx-core/examples/nested_comment.rs`
- `docx-core/examples/numbering.rs`
- `docx-core/examples/outline_lvl.rs`
- `docx-core/examples/page_margin.rs`
- `docx-core/examples/page_size.rs`
- `docx-core/examples/reader.rs`
- `docx-core/examples/sdt.rs`
- `docx-core/examples/section.rs`
- `docx-core/examples/style.rs`
- `docx-core/examples/table_border.rs`
- `docx-core/examples/table.rs`
- `docx-core/examples/toc_simple.rs`
- `docx-core/examples/toc_with_comment.rs`
- `docx-core/examples/toc_with_hyperlink.rs`
- `docx-core/examples/toc_with_item.rs`
- `docx-core/examples/toc_with_style_level.rs`
- `docx-core/examples/toc_with_tc.rs`
- `docx-core/examples/web_ext.rs`

## `docx-core/src/`

The crate root and its shared types.

- `docx-core/src/lib.rs`
- `docx-core/src/macros.rs`

## `docx-core/src/documents/`

The document model: one file per part of a .docx package (document, styles, numbering, settings, comments, headers, footers).

- `docx-core/src/documents/bookmark_id.rs`
- `docx-core/src/documents/build_xml.rs`
- `docx-core/src/documents/comments_extended.rs`
- `docx-core/src/documents/comments.rs`
- `docx-core/src/documents/content_types.rs`
- `docx-core/src/documents/custom_item_property.rs`
- `docx-core/src/documents/custom_item_rels.rs`
- `docx-core/src/documents/custom_item.rs`
- `docx-core/src/documents/document_rels.rs`
- `docx-core/src/documents/document.rs`
- `docx-core/src/documents/font_table.rs`
- `docx-core/src/documents/footer_id.rs`
- `docx-core/src/documents/footer_rels.rs`
- `docx-core/src/documents/footer.rs`
- `docx-core/src/documents/footnote_id.rs`
- `docx-core/src/documents/footnotes.rs`
- `docx-core/src/documents/header_id.rs`
- `docx-core/src/documents/header_rels.rs`
- `docx-core/src/documents/header.rs`
- `docx-core/src/documents/history_id.rs`
- `docx-core/src/documents/hyperlink_id.rs`
- `docx-core/src/documents/image_collector.rs`
- `docx-core/src/documents/mod.rs`
- `docx-core/src/documents/numberings.rs`
- `docx-core/src/documents/paragraph_id.rs`
- `docx-core/src/documents/paragraph_property_change_id.rs`
- `docx-core/src/documents/pic_id.rs`
- `docx-core/src/documents/rels.rs`
- `docx-core/src/documents/settings.rs`
- `docx-core/src/documents/styles.rs`
- `docx-core/src/documents/taskpanes_rels.rs`
- `docx-core/src/documents/taskpanes.rs`
- `docx-core/src/documents/theme.rs`
- `docx-core/src/documents/toc_key.rs`
- `docx-core/src/documents/webextension.rs`
- `docx-core/src/documents/web_settings.rs`
- `docx-core/src/documents/xml_docx.rs`

## `docx-core/src/documents/doc_props/`

DocProps/app.xml, core.xml and custom.xml.

- `docx-core/src/documents/doc_props/app.rs`
- `docx-core/src/documents/doc_props/core.rs`
- `docx-core/src/documents/doc_props/custom.rs`
- `docx-core/src/documents/doc_props/mod.rs`

## `docx-core/src/documents/elements/`

One file per OOXML element the writer builds (runs, paragraphs, tables, drawings, fields, OMML); the fork's lang.rs, paragraph borders and math elements are here.

- `docx-core/src/documents/elements/abstract_numbering.rs`
- `docx-core/src/documents/elements/adjust_right_ind.rs`
- `docx-core/src/documents/elements/a_graphic_data.rs`
- `docx-core/src/documents/elements/a_graphic.rs`
- `docx-core/src/documents/elements/based_on.rs`
- `docx-core/src/documents/elements/bold_cs.rs`
- `docx-core/src/documents/elements/bold.rs`
- `docx-core/src/documents/elements/bookmark_end.rs`
- `docx-core/src/documents/elements/bookmark_start.rs`
- `docx-core/src/documents/elements/br.rs`
- `docx-core/src/documents/elements/cant_split.rs`
- `docx-core/src/documents/elements/caps.rs`
- `docx-core/src/documents/elements/cell_margins.rs`
- `docx-core/src/documents/elements/character_spacing.rs`
- `docx-core/src/documents/elements/color.rs`
- `docx-core/src/documents/elements/comment_extended.rs`
- `docx-core/src/documents/elements/comment_range_end.rs`
- `docx-core/src/documents/elements/comment_range_start.rs`
- `docx-core/src/documents/elements/comment.rs`
- `docx-core/src/documents/elements/cr.rs`
- `docx-core/src/documents/elements/data_binding.rs`
- `docx-core/src/documents/elements/default_tab_stop.rs`
- `docx-core/src/documents/elements/delete_instr_text.rs`
- `docx-core/src/documents/elements/delete.rs`
- `docx-core/src/documents/elements/delete_text.rs`
- `docx-core/src/documents/elements/div.rs`
- `docx-core/src/documents/elements/doc_defaults.rs`
- `docx-core/src/documents/elements/doc_grid.rs`
- `docx-core/src/documents/elements/doc_id.rs`
- `docx-core/src/documents/elements/doc_var.rs`
- `docx-core/src/documents/elements/drawing.rs`
- `docx-core/src/documents/elements/dstrike.rs`
- `docx-core/src/documents/elements/fit_text.rs`
- `docx-core/src/documents/elements/fld_char.rs`
- `docx-core/src/documents/elements/font.rs`
- `docx-core/src/documents/elements/font_scheme.rs`
- `docx-core/src/documents/elements/footer_reference.rs`
- `docx-core/src/documents/elements/footnote_reference.rs`
- `docx-core/src/documents/elements/footnote.rs`
- `docx-core/src/documents/elements/frame_property.rs`
- `docx-core/src/documents/elements/grid_span.rs`
- `docx-core/src/documents/elements/header_reference.rs`
- `docx-core/src/documents/elements/highlight.rs`
- `docx-core/src/documents/elements/hyperlink.rs`
- `docx-core/src/documents/elements/indent_level.rs`
- `docx-core/src/documents/elements/indent.rs`
- `docx-core/src/documents/elements/insert.rs`
- `docx-core/src/documents/elements/instr_hyperlink.rs`
- `docx-core/src/documents/elements/instr_num_pages.rs`
- `docx-core/src/documents/elements/instr_pageref.rs`
- `docx-core/src/documents/elements/instr_page.rs`
- `docx-core/src/documents/elements/instr_tc.rs`
- `docx-core/src/documents/elements/instr_text.rs`
- `docx-core/src/documents/elements/instr_toc.rs`
- `docx-core/src/documents/elements/is_lgl.rs`
- `docx-core/src/documents/elements/italic_cs.rs`
- `docx-core/src/documents/elements/italic.rs`
- `docx-core/src/documents/elements/justification.rs`
- `docx-core/src/documents/elements/lang.rs`
- `docx-core/src/documents/elements/level_jc.rs`
- `docx-core/src/documents/elements/level_override.rs`
- `docx-core/src/documents/elements/level_restart.rs`
- `docx-core/src/documents/elements/level.rs`
- `docx-core/src/documents/elements/level_text.rs`
- `docx-core/src/documents/elements/line_spacing.rs`
- `docx-core/src/documents/elements/link.rs`
- `docx-core/src/documents/elements/mc_fallback.rs`
- `docx-core/src/documents/elements/mod.rs`
- `docx-core/src/documents/elements/move_from.rs`
- `docx-core/src/documents/elements/move_to.rs`
- `docx-core/src/documents/elements/name.rs`
- `docx-core/src/documents/elements/next.rs`
- `docx-core/src/documents/elements/number_format.rs`
- `docx-core/src/documents/elements/numbering_id.rs`
- `docx-core/src/documents/elements/numbering_property.rs`
- `docx-core/src/documents/elements/numbering.rs`
- `docx-core/src/documents/elements/num_pages.rs`
- `docx-core/src/documents/elements/omath.rs`
- `docx-core/src/documents/elements/outline_lvl.rs`
- `docx-core/src/documents/elements/page_margin.rs`
- `docx-core/src/documents/elements/page_num.rs`
- `docx-core/src/documents/elements/page_num_type.rs`
- `docx-core/src/documents/elements/page_size.rs`
- `docx-core/src/documents/elements/paragraph_borders.rs`
- `docx-core/src/documents/elements/paragraph_property_change.rs`
- `docx-core/src/documents/elements/paragraph_property_default.rs`
- `docx-core/src/documents/elements/paragraph_property.rs`
- `docx-core/src/documents/elements/paragraph.rs`
- `docx-core/src/documents/elements/paragraph_style.rs`
- `docx-core/src/documents/elements/pic.rs`
- `docx-core/src/documents/elements/positional_tab.rs`
- `docx-core/src/documents/elements/q_format.rs`
- `docx-core/src/documents/elements/run_fonts.rs`
- `docx-core/src/documents/elements/run_property_default.rs`
- `docx-core/src/documents/elements/run_property.rs`
- `docx-core/src/documents/elements/run.rs`
- `docx-core/src/documents/elements/run_style.rs`
- `docx-core/src/documents/elements/section_property.rs`
- `docx-core/src/documents/elements/section.rs`
- `docx-core/src/documents/elements/shading.rs`
- `docx-core/src/documents/elements/shape.rs`
- `docx-core/src/documents/elements/spec_vanish.rs`
- `docx-core/src/documents/elements/start.rs`
- `docx-core/src/documents/elements/stretch.rs`
- `docx-core/src/documents/elements/strike.rs`
- `docx-core/src/documents/elements/structured_data_tag_property.rs`
- `docx-core/src/documents/elements/structured_data_tag.rs`
- `docx-core/src/documents/elements/style.rs`
- `docx-core/src/documents/elements/sym.rs`
- `docx-core/src/documents/elements/sz_cs.rs`
- `docx-core/src/documents/elements/sz.rs`
- `docx-core/src/documents/elements/table_borders.rs`
- `docx-core/src/documents/elements/table_cell_borders.rs`
- `docx-core/src/documents/elements/table_cell_margins.rs`
- `docx-core/src/documents/elements/table_cell_property.rs`
- `docx-core/src/documents/elements/table_cell.rs`
- `docx-core/src/documents/elements/table_cell_width.rs`
- `docx-core/src/documents/elements/table_grid.rs`
- `docx-core/src/documents/elements/table_indent.rs`
- `docx-core/src/documents/elements/table_layout.rs`
- `docx-core/src/documents/elements/table_of_contents_item.rs`
- `docx-core/src/documents/elements/table_of_contents.rs`
- `docx-core/src/documents/elements/table_position_property.rs`
- `docx-core/src/documents/elements/table_property.rs`
- `docx-core/src/documents/elements/table_row_property.rs`
- `docx-core/src/documents/elements/table_row.rs`
- `docx-core/src/documents/elements/table.rs`
- `docx-core/src/documents/elements/table_style.rs`
- `docx-core/src/documents/elements/table_width.rs`
- `docx-core/src/documents/elements/tab.rs`
- `docx-core/src/documents/elements/tabs.rs`
- `docx-core/src/documents/elements/text_alignment.rs`
- `docx-core/src/documents/elements/text_border.rs`
- `docx-core/src/documents/elements/text_box_content.rs`
- `docx-core/src/documents/elements/text_box.rs`
- `docx-core/src/documents/elements/text_direction.rs`
- `docx-core/src/documents/elements/text.rs`
- `docx-core/src/documents/elements/underline.rs`
- `docx-core/src/documents/elements/v_align.rs`
- `docx-core/src/documents/elements/vanish.rs`
- `docx-core/src/documents/elements/vert_align.rs`
- `docx-core/src/documents/elements/vertical_merge.rs`
- `docx-core/src/documents/elements/wp_anchor.rs`
- `docx-core/src/documents/elements/wps_shape.rs`
- `docx-core/src/documents/elements/wps_text_box.rs`
- `docx-core/src/documents/elements/zoom.rs`

## `docx-core/src/documents/preset_styles/`

The default styles a new document carries.

- `docx-core/src/documents/preset_styles/mod.rs`
- `docx-core/src/documents/preset_styles/toc.rs`

## `docx-core/src/errors/`

The crate's error types.

- `docx-core/src/errors/mod.rs`

## `docx-core/src/escape/`

XML escaping.

- `docx-core/src/escape/mod.rs`

## `docx-core/src/reader/`

The .docx reader: one file per element it parses back into the model.

- `docx-core/src/reader/a_graphic_data.rs`
- `docx-core/src/reader/a_graphic.rs`
- `docx-core/src/reader/bookmark_end.rs`
- `docx-core/src/reader/bookmark_start.rs`
- `docx-core/src/reader/cell_margins.rs`
- `docx-core/src/reader/comment_extended.rs`
- `docx-core/src/reader/comment.rs`
- `docx-core/src/reader/comments_extended.rs`
- `docx-core/src/reader/comments.rs`
- `docx-core/src/reader/custom_properties.rs`
- `docx-core/src/reader/delete.rs`
- `docx-core/src/reader/div.rs`
- `docx-core/src/reader/doc_defaults.rs`
- `docx-core/src/reader/doc_grid.rs`
- `docx-core/src/reader/document_rels.rs`
- `docx-core/src/reader/document.rs`
- `docx-core/src/reader/drawing.rs`
- `docx-core/src/reader/errors.rs`
- `docx-core/src/reader/font_group.rs`
- `docx-core/src/reader/font_scheme.rs`
- `docx-core/src/reader/footer.rs`
- `docx-core/src/reader/frame_property.rs`
- `docx-core/src/reader/from_xml.rs`
- `docx-core/src/reader/header_or_footer_rels.rs`
- `docx-core/src/reader/header.rs`
- `docx-core/src/reader/hyperlink.rs`
- `docx-core/src/reader/ignore.rs`
- `docx-core/src/reader/insert.rs`
- `docx-core/src/reader/level_override.rs`
- `docx-core/src/reader/level.rs`
- `docx-core/src/reader/mc_fallback.rs`
- `docx-core/src/reader/mod.rs`
- `docx-core/src/reader/move_from.rs`
- `docx-core/src/reader/move_to.rs`
- `docx-core/src/reader/namespace.rs`
- `docx-core/src/reader/numbering_property.rs`
- `docx-core/src/reader/numberings.rs`
- `docx-core/src/reader/page_num_type.rs`
- `docx-core/src/reader/paragraph_property_change.rs`
- `docx-core/src/reader/paragraph_property.rs`
- `docx-core/src/reader/paragraph.rs`
- `docx-core/src/reader/pic.rs`
- `docx-core/src/reader/positional_tab.rs`
- `docx-core/src/reader/read_docx.rs`
- `docx-core/src/reader/read_xml.rs`
- `docx-core/src/reader/read_zip.rs`
- `docx-core/src/reader/rels.rs`
- `docx-core/src/reader/run_property.rs`
- `docx-core/src/reader/run.rs`
- `docx-core/src/reader/section_property.rs`
- `docx-core/src/reader/settings.rs`
- `docx-core/src/reader/shading.rs`
- `docx-core/src/reader/shape.rs`
- `docx-core/src/reader/structured_data_tag.rs`
- `docx-core/src/reader/style.rs`
- `docx-core/src/reader/styles.rs`
- `docx-core/src/reader/table_borders.rs`
- `docx-core/src/reader/table_cell_borders.rs`
- `docx-core/src/reader/table_cell_margins.rs`
- `docx-core/src/reader/table_cell_property.rs`
- `docx-core/src/reader/table_cell.rs`
- `docx-core/src/reader/table_position_property.rs`
- `docx-core/src/reader/table_property.rs`
- `docx-core/src/reader/table_row.rs`
- `docx-core/src/reader/table.rs`
- `docx-core/src/reader/tab.rs`
- `docx-core/src/reader/tabs.rs`
- `docx-core/src/reader/text_box_content.rs`
- `docx-core/src/reader/theme.rs`
- `docx-core/src/reader/web_settings.rs`
- `docx-core/src/reader/wp_anchor.rs`
- `docx-core/src/reader/wps_shape.rs`
- `docx-core/src/reader/wps_text_box.rs`
- `docx-core/src/reader/xml_element.rs`
- `docx-core/src/reader/xml_parser.rs`
- `docx-core/src/reader/xml_reader.rs`

## `docx-core/src/reader/attributes/`

Attribute parsers the reader shares.

- `docx-core/src/reader/attributes/bool_value.rs`
- `docx-core/src/reader/attributes/border.rs`
- `docx-core/src/reader/attributes/id.rs`
- `docx-core/src/reader/attributes/indent_level.rs`
- `docx-core/src/reader/attributes/indent.rs`
- `docx-core/src/reader/attributes/line_spacing.rs`
- `docx-core/src/reader/attributes/mod.rs`
- `docx-core/src/reader/attributes/name.rs`
- `docx-core/src/reader/attributes/val.rs`
- `docx-core/src/reader/attributes/width.rs`

## `docx-core/src/types/`

OOXML simple types (ST_*) as Rust enums.

- `docx-core/src/types/alignment_type.rs`
- `docx-core/src/types/border_position.rs`
- `docx-core/src/types/border_type.rs`
- `docx-core/src/types/break_type.rs`
- `docx-core/src/types/character_spacing_values.rs`
- `docx-core/src/types/doc_grid_type.rs`
- `docx-core/src/types/drawing_position.rs`
- `docx-core/src/types/emu.rs`
- `docx-core/src/types/errors.rs`
- `docx-core/src/types/field_char_type.rs`
- `docx-core/src/types/font_pitch_type.rs`
- `docx-core/src/types/height_rule.rs`
- `docx-core/src/types/hyperlink_type.rs`
- `docx-core/src/types/level_suffix_type.rs`
- `docx-core/src/types/line_spacing_type.rs`
- `docx-core/src/types/mod.rs`
- `docx-core/src/types/page_margin.rs`
- `docx-core/src/types/page_orientation_type.rs`
- `docx-core/src/types/positional_tab_alignment_type.rs`
- `docx-core/src/types/positional_tab_relative_to.rs`
- `docx-core/src/types/relative_from_type.rs`
- `docx-core/src/types/section_type.rs`
- `docx-core/src/types/shd_type.rs`
- `docx-core/src/types/special_indent_type.rs`
- `docx-core/src/types/style_type.rs`
- `docx-core/src/types/tab_leader_type.rs`
- `docx-core/src/types/table_alignment_type.rs`
- `docx-core/src/types/table_layout_type.rs`
- `docx-core/src/types/tab_value_type.rs`
- `docx-core/src/types/text_alignment_type.rs`
- `docx-core/src/types/text_direction_type.rs`
- `docx-core/src/types/vert_align_type.rs`
- `docx-core/src/types/vertical_align_type.rs`
- `docx-core/src/types/vertical_merge_type.rs`
- `docx-core/src/types/width_type.rs`

## `docx-core/src/xml/`

The XML tree the reader walks.

- `docx-core/src/xml/common.rs`
- `docx-core/src/xml/mod.rs`
- `docx-core/src/xml/writer.rs`

## `docx-core/src/xml_builder/`

The XML writer: one file per part, and the element macros.

- `docx-core/src/xml_builder/comments_extended.rs`
- `docx-core/src/xml_builder/comments.rs`
- `docx-core/src/xml_builder/core_properties.rs`
- `docx-core/src/xml_builder/custom_properties.rs`
- `docx-core/src/xml_builder/declaration.rs`
- `docx-core/src/xml_builder/document.rs`
- `docx-core/src/xml_builder/drawing.rs`
- `docx-core/src/xml_builder/elements.rs`
- `docx-core/src/xml_builder/fonts.rs`
- `docx-core/src/xml_builder/footer.rs`
- `docx-core/src/xml_builder/footnotes.rs`
- `docx-core/src/xml_builder/header.rs`
- `docx-core/src/xml_builder/macros.rs`
- `docx-core/src/xml_builder/mod.rs`
- `docx-core/src/xml_builder/numbering.rs`
- `docx-core/src/xml_builder/pic.rs`
- `docx-core/src/xml_builder/properties.rs`
- `docx-core/src/xml_builder/relationship.rs`
- `docx-core/src/xml_builder/settings.rs`
- `docx-core/src/xml_builder/styles.rs`

## `docx-core/src/xml_json/`

XML to JSON for the reader's JSON output.

- `docx-core/src/xml_json/mod.rs`

## `docx-core/src/zipper/`

Packs the parts into the .docx zip.

- `docx-core/src/zipper/mod.rs`

## `docx-core/tests/`

Integration tests: writing and reading whole documents.

- `docx-core/tests/lib.rs`
- `docx-core/tests/reader.rs`

## `docx-wasm/`

The npm package docx-wasm: its manifests, TypeScript and webpack settings and entry; not used by the pair.

- `docx-wasm/Cargo.toml`
- `docx-wasm/index.js`
- `docx-wasm/.npmignore`
- `docx-wasm/package.json`
- `docx-wasm/pnpm-lock.yaml`
- `docx-wasm/tsconfig.json`
- `docx-wasm/tsconfig.node.json`
- `docx-wasm/tsconfig.web.json`
- `docx-wasm/webpack.common.js`
- `docx-wasm/webpack.dev.js`
- `docx-wasm/webpack.prod.js`

## `docx-wasm/assets/`

The HTML template the demo page is built from.

- `docx-wasm/assets/template.html`

## `docx-wasm/example/`

A TypeScript example of the binding.

- `docx-wasm/example/index.ts`

## `docx-wasm/export-png/`

Upstream's visual regression: renders the test documents to PNG.

- `docx-wasm/export-png/.gitignore`
- `docx-wasm/export-png/index.mjs`
- `docx-wasm/export-png/.keep`
- `docx-wasm/export-png/makefile`
- `docx-wasm/export-png/tsconfig.json`

## `docx-wasm/export-png/png/`

Upstream's visual regression: renders the test documents to PNG.

- `docx-wasm/export-png/png/.keep`

## `docx-wasm/js/`

The binding's TypeScript API, one file per element.

- `docx-wasm/js/abstract-numbering.ts`
- `docx-wasm/js/bookmark-end.ts`
- `docx-wasm/js/bookmark-start.ts`
- `docx-wasm/js/border.ts`
- `docx-wasm/js/break.ts`
- `docx-wasm/js/builder.ts`
- `docx-wasm/js/carriage-return.ts`
- `docx-wasm/js/comment-end.ts`
- `docx-wasm/js/comment.ts`
- `docx-wasm/js/delete-text.ts`
- `docx-wasm/js/delete.ts`
- `docx-wasm/js/doc-defaults.ts`
- `docx-wasm/js/doc-props.ts`
- `docx-wasm/js/footer.ts`
- `docx-wasm/js/header.ts`
- `docx-wasm/js/hyperlink.ts`
- `docx-wasm/js/image.ts`
- `docx-wasm/js/index.ts`
- `docx-wasm/js/insert.ts`
- `docx-wasm/js/level.ts`
- `docx-wasm/js/numbering.ts`
- `docx-wasm/js/num-pages.ts`
- `docx-wasm/js/page-num.ts`
- `docx-wasm/js/paragraph-property.ts`
- `docx-wasm/js/paragraph.ts`
- `docx-wasm/js/positional-tab.ts`
- `docx-wasm/js/run-property.ts`
- `docx-wasm/js/run.ts`
- `docx-wasm/js/section-property.ts`
- `docx-wasm/js/settings.ts`
- `docx-wasm/js/shading.ts`
- `docx-wasm/js/styles.ts`
- `docx-wasm/js/style.ts`
- `docx-wasm/js/tab-leader.ts`
- `docx-wasm/js/table-cell-borders.ts`
- `docx-wasm/js/table-cell-border.ts`
- `docx-wasm/js/table-cell.ts`
- `docx-wasm/js/table-of-contents-item.ts`
- `docx-wasm/js/table-of-contents.ts`
- `docx-wasm/js/table-row.ts`
- `docx-wasm/js/table.ts`
- `docx-wasm/js/tab.ts`
- `docx-wasm/js/tc.ts`
- `docx-wasm/js/text.ts`
- `docx-wasm/js/webextension.ts`

## `docx-wasm/js/json/`

TypeScript types of the reader's JSON output.

- `docx-wasm/js/json/border.ts`
- `docx-wasm/js/json/comment.ts`
- `docx-wasm/js/json/document.ts`
- `docx-wasm/js/json/drawing.ts`
- `docx-wasm/js/json/footer.ts`
- `docx-wasm/js/json/header.ts`
- `docx-wasm/js/json/indent.ts`
- `docx-wasm/js/json/index.ts`
- `docx-wasm/js/json/line_spacing.ts`
- `docx-wasm/js/json/numbering.ts`
- `docx-wasm/js/json/paragraph.ts`
- `docx-wasm/js/json/run.ts`
- `docx-wasm/js/json/section-property.ts`
- `docx-wasm/js/json/shading.ts`
- `docx-wasm/js/json/shape.ts`
- `docx-wasm/js/json/structured-data-tag.ts`
- `docx-wasm/js/json/styles.ts`
- `docx-wasm/js/json/table.ts`
- `docx-wasm/js/json/textbox-content.ts`
- `docx-wasm/js/json/web-settings.ts`

## `docx-wasm/js/json/bindings/`

TypeScript types ts-rs generates for the JSON output.

- `docx-wasm/js/json/bindings/AlignmentType.ts`
- `docx-wasm/js/json/bindings/BoldCs.ts`
- `docx-wasm/js/json/bindings/Bold.ts`
- `docx-wasm/js/json/bindings/BorderType.ts`
- `docx-wasm/js/json/bindings/Break.ts`
- `docx-wasm/js/json/bindings/BreakType.ts`
- `docx-wasm/js/json/bindings/Caps.ts`
- `docx-wasm/js/json/bindings/CharacterSpacing.ts`
- `docx-wasm/js/json/bindings/Color.ts`
- `docx-wasm/js/json/bindings/CommentRangeEnd.ts`
- `docx-wasm/js/json/bindings/CommentRangeStart.ts`
- `docx-wasm/js/json/bindings/Comment.ts`
- `docx-wasm/js/json/bindings/DeleteChild.ts`
- `docx-wasm/js/json/bindings/DeleteText.ts`
- `docx-wasm/js/json/bindings/Delete.ts`
- `docx-wasm/js/json/bindings/DrawingPosition.ts`
- `docx-wasm/js/json/bindings/DrawingPositionType.ts`
- `docx-wasm/js/json/bindings/FieldChar.ts`
- `docx-wasm/js/json/bindings/FieldCharType.ts`
- `docx-wasm/js/json/bindings/FitText.ts`
- `docx-wasm/js/json/bindings/FontGroup.ts`
- `docx-wasm/js/json/bindings/FontSchemeFont.ts`
- `docx-wasm/js/json/bindings/FontScheme.ts`
- `docx-wasm/js/json/bindings/FooterReference.ts`
- `docx-wasm/js/json/bindings/FrameProperty.ts`
- `docx-wasm/js/json/bindings/HeaderReference.ts`
- `docx-wasm/js/json/bindings/Highlight.ts`
- `docx-wasm/js/json/bindings/HyperlinkType.ts`
- `docx-wasm/js/json/bindings/ImageData.ts`
- `docx-wasm/js/json/bindings/InsertChild.ts`
- `docx-wasm/js/json/bindings/Insert.ts`
- `docx-wasm/js/json/bindings/InstrHyperlink.ts`
- `docx-wasm/js/json/bindings/InstrPAGEREF.ts`
- `docx-wasm/js/json/bindings/InstrTC.ts`
- `docx-wasm/js/json/bindings/InstrText.ts`
- `docx-wasm/js/json/bindings/InstrToC.ts`
- `docx-wasm/js/json/bindings/ItalicCs.ts`
- `docx-wasm/js/json/bindings/Italic.ts`
- `docx-wasm/js/json/bindings/PageNumType.ts`
- `docx-wasm/js/json/bindings/PicAlign.ts`
- `docx-wasm/js/json/bindings/Pic.ts`
- `docx-wasm/js/json/bindings/PositionalTabAlignmentType.ts`
- `docx-wasm/js/json/bindings/PositionalTabRelativeTo.ts`
- `docx-wasm/js/json/bindings/PositionalTab.ts`
- `docx-wasm/js/json/bindings/RelativeFromHType.ts`
- `docx-wasm/js/json/bindings/RelativeFromVType.ts`
- `docx-wasm/js/json/bindings/RunFonts.ts`
- `docx-wasm/js/json/bindings/RunProperty.ts`
- `docx-wasm/js/json/bindings/RunStyle.ts`
- `docx-wasm/js/json/bindings/Run.ts`
- `docx-wasm/js/json/bindings/Shape.ts`
- `docx-wasm/js/json/bindings/SpecVanish.ts`
- `docx-wasm/js/json/bindings/Strike.ts`
- `docx-wasm/js/json/bindings/StyleWithLevel.ts`
- `docx-wasm/js/json/bindings/Sym.ts`
- `docx-wasm/js/json/bindings/SzCs.ts`
- `docx-wasm/js/json/bindings/Sz.ts`
- `docx-wasm/js/json/bindings/TabLeaderType.ts`
- `docx-wasm/js/json/bindings/TableCellBorderPosition.ts`
- `docx-wasm/js/json/bindings/TableCellBorders.ts`
- `docx-wasm/js/json/bindings/TableCellBorder.ts`
- `docx-wasm/js/json/bindings/TablePositionProperty.ts`
- `docx-wasm/js/json/bindings/Tab.ts`
- `docx-wasm/js/json/bindings/TabValueType.ts`
- `docx-wasm/js/json/bindings/TextAlignmentType.ts`
- `docx-wasm/js/json/bindings/TextBorder.ts`
- `docx-wasm/js/json/bindings/Text.ts`
- `docx-wasm/js/json/bindings/Theme.ts`
- `docx-wasm/js/json/bindings/Underline.ts`
- `docx-wasm/js/json/bindings/Vanish.ts`
- `docx-wasm/js/json/bindings/VertAlign.ts`

## `docx-wasm/src/`

The Rust side of the binding, compiled to WebAssembly.

- `docx-wasm/src/abstract_numbering.rs`
- `docx-wasm/src/comment.rs`
- `docx-wasm/src/delete.rs`
- `docx-wasm/src/doc.rs`
- `docx-wasm/src/footer.rs`
- `docx-wasm/src/frame_property.rs`
- `docx-wasm/src/header.rs`
- `docx-wasm/src/hyperlink.rs`
- `docx-wasm/src/insert.rs`
- `docx-wasm/src/level_override.rs`
- `docx-wasm/src/level.rs`
- `docx-wasm/src/lib.rs`
- `docx-wasm/src/line_spacing.rs`
- `docx-wasm/src/move_from.rs`
- `docx-wasm/src/move_to.rs`
- `docx-wasm/src/numbering.rs`
- `docx-wasm/src/num_pages.rs`
- `docx-wasm/src/page_margin.rs`
- `docx-wasm/src/page_num.rs`
- `docx-wasm/src/page_num_type.rs`
- `docx-wasm/src/paragraph_property.rs`
- `docx-wasm/src/paragraph.rs`
- `docx-wasm/src/pic.rs`
- `docx-wasm/src/positional_tab.rs`
- `docx-wasm/src/reader.rs`
- `docx-wasm/src/run_fonts.rs`
- `docx-wasm/src/run_property.rs`
- `docx-wasm/src/run.rs`
- `docx-wasm/src/style.rs`
- `docx-wasm/src/table_cell_border.rs`
- `docx-wasm/src/table_cell.rs`
- `docx-wasm/src/table_of_contents_item.rs`
- `docx-wasm/src/table_of_contents.rs`
- `docx-wasm/src/table_position_property.rs`
- `docx-wasm/src/table_row.rs`
- `docx-wasm/src/table.rs`
- `docx-wasm/src/web_extension.rs`

## `docx-wasm/src/adaptors/`

Conversions between the binding's and the crate's types.

- `docx-wasm/src/adaptors/mod.rs`
- `docx-wasm/src/adaptors/special_indent.rs`

## `docx-wasm/test/`

The binding's Jest tests and the folder they write into.

- `docx-wasm/test/cat.js`
- `docx-wasm/test/encoded-cat.js`
- `docx-wasm/test/index.test.js`

## `docx-wasm/test/output/`

The binding's Jest tests and the folder they write into.

- `docx-wasm/test/output/.keep`
