# Files of upstream's trees

Referenced by: [`MASTER.md`](MASTER.md), which maps these trees by directory. This index lists
every tracked file under `docx-core/`, `docx-wasm/`, `fixtures/` and `docs/`, under its directory.
It is written by [`scripts/files-index.sh`](scripts/files-index.sh) from `git ls-files`; never edit
it by hand. Run `sh scripts/files-index.sh` after an upstream merge or after adding or removing a
file there, and `sh scripts/files-index.sh --check` to see whether it is current.

The data (the fixtures, one section each; the snapshots; the test output; the built demo page) is
listed too, so every file is mapped until check-graph accepts a folder row.

## `docs/`

Data: upstream's built demo page, webpack bundles and the wasm module.

- `docs/0.index.js`
- `docs/1.index.js`
- `docs/cf1f90b2feef7ead7222.module.wasm`
- `docs/index.html`
- `docs/index.js`

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

## `docx-core/src/documents/snapshots/`

Data: snapshots the tests compare against (insta for Rust, Jest for the binding).

- `docx-core/src/documents/snapshots/docx_rs__documents__comments_extended__tests__comments_extended_snapshot.snap`
- `docx-core/src/documents/snapshots/docx_rs__documents__comments_extended__tests__settings.snap`

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

## `docx-core/tests/output/`

Data: a written document's parts, kept by upstream as test output.

- `docx-core/tests/output/[Content_Types].xml`
- `docx-core/tests/output/docProps/app.xml`
- `docx-core/tests/output/docProps/core.xml`
- `docx-core/tests/output/.keep`
- `docx-core/tests/output/_rels/.rels`
- `docx-core/tests/output/word/comments.xml`
- `docx-core/tests/output/word/document.xml`
- `docx-core/tests/output/word/fontTable.xml`
- `docx-core/tests/output/word/numberings.xml`
- `docx-core/tests/output/word/_rels/document.xml.rels`
- `docx-core/tests/output/word/settings.xml`
- `docx-core/tests/output/word/styles.xml`

## `docx-core/tests/snapshots/`

Data: snapshots the tests compare against (insta for Rust, Jest for the binding).

- `docx-core/tests/snapshots/lib__reader__line_spacing.snap`
- `docx-core/tests/snapshots/lib__reader__read_bom.snap`
- `docx-core/tests/snapshots/lib__reader__read_bookmark.snap`
- `docx-core/tests/snapshots/lib__reader__read_comment_in_delete_in_insert.snap`
- `docx-core/tests/snapshots/lib__reader__read_comment.snap`
- `docx-core/tests/snapshots/lib__reader__read_decoration.snap`
- `docx-core/tests/snapshots/lib__reader__read_extended_comments.snap`
- `docx-core/tests/snapshots/lib__reader__read_footnotes.snap`
- `docx-core/tests/snapshots/lib__reader__read_from_doc.snap`
- `docx-core/tests/snapshots/lib__reader__read_hello.snap`
- `docx-core/tests/snapshots/lib__reader__read_highlight_and_underline.snap`
- `docx-core/tests/snapshots/lib__reader__read_history.snap`
- `docx-core/tests/snapshots/lib__reader__read_indent_word_online.snap`
- `docx-core/tests/snapshots/lib__reader__read_insert_table.snap`
- `docx-core/tests/snapshots/lib__reader__read_lvl_override.snap`
- `docx-core/tests/snapshots/lib__reader__read_numbering.snap`
- `docx-core/tests/snapshots/lib__reader__read_tab_and_break.snap`
- `docx-core/tests/snapshots/lib__reader__read_table_docx.snap`
- `docx-core/tests/snapshots/lib__reader__read_table_merged_libre_office.snap`
- `docx-core/tests/snapshots/lib__reader__read_textbox.snap`
- `docx-core/tests/snapshots/reader__line_spacing.snap`
- `docx-core/tests/snapshots/reader__read_bom.snap`
- `docx-core/tests/snapshots/reader__read_bookmark.snap`
- `docx-core/tests/snapshots/reader__read_comment_in_delete_in_insert.snap`
- `docx-core/tests/snapshots/reader__read_comment.snap`
- `docx-core/tests/snapshots/reader__read_decoration.snap`
- `docx-core/tests/snapshots/reader__read_extended_comments.snap`
- `docx-core/tests/snapshots/reader__read_footnotes.snap`
- `docx-core/tests/snapshots/reader__read_from_doc.snap`
- `docx-core/tests/snapshots/reader__read_hello.snap`
- `docx-core/tests/snapshots/reader__read_highlight_and_underline.snap`
- `docx-core/tests/snapshots/reader__read_history.snap`
- `docx-core/tests/snapshots/reader__read_indent_word_online.snap`
- `docx-core/tests/snapshots/reader__read_insert_table.snap`
- `docx-core/tests/snapshots/reader__read_lvl_override.snap`
- `docx-core/tests/snapshots/reader__read_numbering.snap`
- `docx-core/tests/snapshots/reader__read_tab_and_break.snap`
- `docx-core/tests/snapshots/reader__read_table_docx.snap`
- `docx-core/tests/snapshots/reader__read_table_merged_libre_office.snap`
- `docx-core/tests/snapshots/reader__read_textbox.snap`

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

## `docx-wasm/test/__snapshots__/`

Data: snapshots the tests compare against (insta for Rust, Jest for the binding).

- `docx-wasm/test/__snapshots__/index.test.js.snap`

## `fixtures/after_lines/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/after_lines/after_lines.docx`

## `fixtures/bom/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/bom/bom.docx`

## `fixtures/bookmark/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/bookmark/bookmark.docx`
- `fixtures/bookmark/[Content_Types].xml`
- `fixtures/bookmark/docProps/app.xml`
- `fixtures/bookmark/docProps/core.xml`
- `fixtures/bookmark/_rels/.rels`
- `fixtures/bookmark/word/document.xml`
- `fixtures/bookmark/word/fontTable.xml`
- `fixtures/bookmark/word/_rels/document.xml.rels`
- `fixtures/bookmark/word/settings.xml`
- `fixtures/bookmark/word/styles.xml`

## `fixtures/comment/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/comment/cat.jpeg`
- `fixtures/comment/comment.docx`
- `fixtures/comment/[Content_Types].xml`
- `fixtures/comment/docProps/app.xml`
- `fixtures/comment/docProps/core.xml`
- `fixtures/comment/_rels/.rels`
- `fixtures/comment/word/comments.xml`
- `fixtures/comment/word/document.xml`
- `fixtures/comment/word/fontTable.xml`
- `fixtures/comment/word/_rels/document.xml.rels`
- `fixtures/comment/word/settings.xml`
- `fixtures/comment/word/styles.xml`

## `fixtures/comment_in_delete_in_insert/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/comment_in_delete_in_insert/comment_in_delete_in_insert.docx`

## `fixtures/custom/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/custom/custom.docx`

## `fixtures/decoration/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/decoration/[Content_Types].xml`
- `fixtures/decoration/decoration.docx`
- `fixtures/decoration/docProps/app.xml`
- `fixtures/decoration/docProps/core.xml`
- `fixtures/decoration/_rels/.rels`
- `fixtures/decoration/word/document.xml`
- `fixtures/decoration/word/fontTable.xml`
- `fixtures/decoration/word/_rels/document.xml.rels`
- `fixtures/decoration/word/settings.xml`
- `fixtures/decoration/word/styles.xml`

## `fixtures/default_line_spacing/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/default_line_spacing/default_line_spacing.docx`

## `fixtures/del_in_ins/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/del_in_ins/del_in_ins.docx`

## `fixtures/div/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/div/[Content_Types].xml`
- `fixtures/div/div.docx`
- `fixtures/div/docProps/app.xml`
- `fixtures/div/docProps/core.xml`
- `fixtures/div/_rels/.rels`
- `fixtures/div/word/document.xml`
- `fixtures/div/word/fontTable.xml`
- `fixtures/div/word/_rels/document.xml.rels`
- `fixtures/div/word/settings.xml`
- `fixtures/div/word/styles.xml`
- `fixtures/div/word/theme/theme1.xml`
- `fixtures/div/word/webextensions/_rels/taskpanes.xml.rels`
- `fixtures/div/word/webextensions/taskpanes.xml`
- `fixtures/div/word/webextensions/webextension1.xml`
- `fixtures/div/word/webSettings.xml`

## `fixtures/dstrike/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/dstrike/dstrike.docx`

## `fixtures/extended_comments/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/extended_comments/[Content_Types].xml`
- `fixtures/extended_comments/docProps/app.xml`
- `fixtures/extended_comments/docProps/core.xml`
- `fixtures/extended_comments/extended_comments.docx`
- `fixtures/extended_comments/_rels/.rels`
- `fixtures/extended_comments/word/commentsExtended.xml`
- `fixtures/extended_comments/word/comments.xml`
- `fixtures/extended_comments/word/document.xml`
- `fixtures/extended_comments/word/fontTable.xml`
- `fixtures/extended_comments/word/header1.xml`
- `fixtures/extended_comments/word/numbering.xml`
- `fixtures/extended_comments/word/_rels/document.xml.rels`
- `fixtures/extended_comments/word/settings.xml`
- `fixtures/extended_comments/word/styles.xml`

## `fixtures/first_even_header/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/first_even_header/first_even_header.docx`

## `fixtures/font/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/font/font.docx`

## `fixtures/footer/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/footer/footer.docx`

## `fixtures/footnotes/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/footnotes/footnotes.docx`

## `fixtures/from_doc/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/from_doc/from_doc.docx`

## `fixtures/grid_after/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/grid_after/grid_after.docx`

## `fixtures/header_footer/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/header_footer/[Content_Types].xml`
- `fixtures/header_footer/docProps/app.xml`
- `fixtures/header_footer/docProps/core.xml`
- `fixtures/header_footer/header_footer.docx`
- `fixtures/header_footer/_rels/.rels`
- `fixtures/header_footer/word/document.xml`
- `fixtures/header_footer/word/fontTable.xml`
- `fixtures/header_footer/word/footer1.xml`
- `fixtures/header_footer/word/header1.xml`
- `fixtures/header_footer/word/_rels/document.xml.rels`
- `fixtures/header_footer/word/settings.xml`
- `fixtures/header_footer/word/styles.xml`

## `fixtures/hello_libre_office/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/hello_libre_office/[Content_Types].xml`
- `fixtures/hello_libre_office/docProps/app.xml`
- `fixtures/hello_libre_office/docProps/core.xml`
- `fixtures/hello_libre_office/hello.docx`
- `fixtures/hello_libre_office/_rels/.rels`
- `fixtures/hello_libre_office/word/document.xml`
- `fixtures/hello_libre_office/word/fontTable.xml`
- `fixtures/hello_libre_office/word/_rels/document.xml.rels`
- `fixtures/hello_libre_office/word/settings.xml`
- `fixtures/hello_libre_office/word/styles.xml`

## `fixtures/hello_world/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/hello_world/[Content_Types].xml`
- `fixtures/hello_world/docProps/app.xml`
- `fixtures/hello_world/docProps/core.xml`
- `fixtures/hello_world/hello_world.docx`
- `fixtures/hello_world/_rels/.rels`
- `fixtures/hello_world/word/document.xml`
- `fixtures/hello_world/word/footer1.xml`
- `fixtures/hello_world/word/footnotes.xml`
- `fixtures/hello_world/word/header1.xml`
- `fixtures/hello_world/word/numbering.xml`
- `fixtures/hello_world/word/_rels/footer1.xml.rels`
- `fixtures/hello_world/word/_rels/header1.xml.rels`
- `fixtures/hello_world/word/styles.xml`

## `fixtures/hidden/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/hidden/[Content_Types].xml`
- `fixtures/hidden/docProps/app.xml`
- `fixtures/hidden/docProps/core.xml`
- `fixtures/hidden/hidden.docx`
- `fixtures/hidden/_rels/.rels`
- `fixtures/hidden/word/document.xml`
- `fixtures/hidden/word/fontTable.xml`
- `fixtures/hidden/word/_rels/document.xml.rels`
- `fixtures/hidden/word/settings.xml`
- `fixtures/hidden/word/styles.xml`

## `fixtures/highlight_and_underline/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/highlight_and_underline/[Content_Types].xml`
- `fixtures/highlight_and_underline/docProps/app.xml`
- `fixtures/highlight_and_underline/docProps/core.xml`
- `fixtures/highlight_and_underline/highlight_and_underline.docx`
- `fixtures/highlight_and_underline/_rels/.rels`
- `fixtures/highlight_and_underline/word/document.xml`
- `fixtures/highlight_and_underline/word/fontTable.xml`
- `fixtures/highlight_and_underline/word/_rels/document.xml.rels`
- `fixtures/highlight_and_underline/word/settings.xml`
- `fixtures/highlight_and_underline/word/styles.xml`

## `fixtures/history_libre_office/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/history_libre_office/[Content_Types].xml`
- `fixtures/history_libre_office/docProps/app.xml`
- `fixtures/history_libre_office/docProps/core.xml`
- `fixtures/history_libre_office/history.docx`
- `fixtures/history_libre_office/_rels/.rels`
- `fixtures/history_libre_office/word/document.xml`
- `fixtures/history_libre_office/word/fontTable.xml`
- `fixtures/history_libre_office/word/_rels/document.xml.rels`
- `fixtures/history_libre_office/word/settings.xml`
- `fixtures/history_libre_office/word/styles.xml`

## `fixtures/image/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image/[Content_Types].xml`
- `fixtures/image/docProps/app.xml`
- `fixtures/image/docProps/core.xml`
- `fixtures/image/image.docx`
- `fixtures/image/_rels/.rels`
- `fixtures/image/word/document.xml`
- `fixtures/image/word/fontTable.xml`
- `fixtures/image/word/media/image1.png`
- `fixtures/image/word/_rels/document.xml.rels`
- `fixtures/image/word/settings.xml`
- `fixtures/image/word/styles.xml`

## `fixtures/image_inline_and_anchor/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_inline_and_anchor/image_inline_and_anchor.docx`

## `fixtures/image_in_textbox/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_in_textbox/[Content_Types].xml`
- `fixtures/image_in_textbox/docProps/app.xml`
- `fixtures/image_in_textbox/docProps/core.xml`
- `fixtures/image_in_textbox/image_in_textbox.docx`
- `fixtures/image_in_textbox/_rels/.rels`
- `fixtures/image_in_textbox/word/document.xml`
- `fixtures/image_in_textbox/word/fontTable.xml`
- `fixtures/image_in_textbox/word/media/image1.png`
- `fixtures/image_in_textbox/word/_rels/document.xml.rels`
- `fixtures/image_in_textbox/word/settings.xml`
- `fixtures/image_in_textbox/word/styles.xml`
- `fixtures/image_in_textbox/word/theme/theme1.xml`
- `fixtures/image_in_textbox/word/webSettings.xml`

## `fixtures/image_node_docx/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_node_docx/[Content_Types].xml`
- `fixtures/image_node_docx/docProps/app.xml`
- `fixtures/image_node_docx/docProps/core.xml`
- `fixtures/image_node_docx/image.docx`
- `fixtures/image_node_docx/_rels/.rels`
- `fixtures/image_node_docx/word/document.xml`
- `fixtures/image_node_docx/word/footer1.xml`
- `fixtures/image_node_docx/word/footnotes.xml`
- `fixtures/image_node_docx/word/header1.xml`
- `fixtures/image_node_docx/word/media/2mhefq5b7fsoxmjaoiuyb.png`
- `fixtures/image_node_docx/word/media/40yrczdu6ohoeqs2cg2n4h.png`
- `fixtures/image_node_docx/word/media/oja94skc7s5oq9avc9zyf.png`
- `fixtures/image_node_docx/word/media/vpvf42pjbjmb5zjs1nsbb.png`
- `fixtures/image_node_docx/word/numbering.xml`
- `fixtures/image_node_docx/word/_rels/document.xml.rels`
- `fixtures/image_node_docx/word/_rels/footer1.xml.rels`
- `fixtures/image_node_docx/word/_rels/header1.xml.rels`
- `fixtures/image_node_docx/word/settings.xml`
- `fixtures/image_node_docx/word/styles.xml`

## `fixtures/image_node_docx_floating/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_node_docx_floating/[Content_Types].xml`
- `fixtures/image_node_docx_floating/docProps/app.xml`
- `fixtures/image_node_docx_floating/docProps/core.xml`
- `fixtures/image_node_docx_floating/image.docx`
- `fixtures/image_node_docx_floating/_rels/.rels`
- `fixtures/image_node_docx_floating/word/document.xml`
- `fixtures/image_node_docx_floating/word/footer1.xml`
- `fixtures/image_node_docx_floating/word/footnotes.xml`
- `fixtures/image_node_docx_floating/word/header1.xml`
- `fixtures/image_node_docx_floating/word/media/dy29bt3j1idb692k2uzxlj.png`
- `fixtures/image_node_docx_floating/word/numbering.xml`
- `fixtures/image_node_docx_floating/word/_rels/document.xml.rels`
- `fixtures/image_node_docx_floating/word/_rels/footer1.xml.rels`
- `fixtures/image_node_docx_floating/word/_rels/header1.xml.rels`
- `fixtures/image_node_docx_floating/word/settings.xml`
- `fixtures/image_node_docx_floating/word/styles.xml`

## `fixtures/image_node_docx_relative/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_node_docx_relative/[Content_Types].xml`
- `fixtures/image_node_docx_relative/docProps/app.xml`
- `fixtures/image_node_docx_relative/docProps/core.xml`
- `fixtures/image_node_docx_relative/image.docx`
- `fixtures/image_node_docx_relative/_rels/.rels`
- `fixtures/image_node_docx_relative/word/document.xml`
- `fixtures/image_node_docx_relative/word/footer1.xml`
- `fixtures/image_node_docx_relative/word/footnotes.xml`
- `fixtures/image_node_docx_relative/word/header1.xml`
- `fixtures/image_node_docx_relative/word/media/b7yx0qt3xx9yzh99bhv8d.png`
- `fixtures/image_node_docx_relative/word/numbering.xml`
- `fixtures/image_node_docx_relative/word/_rels/document.xml`
- `fixtures/image_node_docx_relative/word/_rels/footer1.xml.rels`
- `fixtures/image_node_docx_relative/word/_rels/header1.xml.rels`
- `fixtures/image_node_docx_relative/word/settings.xml`
- `fixtures/image_node_docx_relative/word/styles.xml`

## `fixtures/image_output/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_output/image.docx`

## `fixtures/image_output_resized/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_output_resized/[Content_Types].xml`
- `fixtures/image_output_resized/docProps/app.xml`
- `fixtures/image_output_resized/docProps/core.xml`
- `fixtures/image_output_resized/image.docx`
- `fixtures/image_output_resized/_rels/.rels`
- `fixtures/image_output_resized/word/comments.xml`
- `fixtures/image_output_resized/word/document.xml`
- `fixtures/image_output_resized/word/fontTable.xml`
- `fixtures/image_output_resized/word/media/image1.jpg`
- `fixtures/image_output_resized/word/numbering.xml`
- `fixtures/image_output_resized/word/_rels/document.xml.rels`
- `fixtures/image_output_resized/word/settings.xml`
- `fixtures/image_output_resized/word/styles.xml`
- `fixtures/image_output_resized/word/theme/theme1.xml`
- `fixtures/image_output_resized/word/webSettings.xml`

## `fixtures/image_xml/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/image_xml/image.xml`

## `fixtures/indent_word_online/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/indent_word_online/[Content_Types].xml`
- `fixtures/indent_word_online/docProps/app.xml`
- `fixtures/indent_word_online/docProps/core.xml`
- `fixtures/indent_word_online/indent.docx`
- `fixtures/indent_word_online/_rels/.rels`
- `fixtures/indent_word_online/word/document.xml`
- `fixtures/indent_word_online/word/fontTable.xml`
- `fixtures/indent_word_online/word/_rels/document2.xml.rels`
- `fixtures/indent_word_online/word/_rels/document.xml.rels`
- `fixtures/indent_word_online/word/settings.xml`
- `fixtures/indent_word_online/word/styles.xml`
- `fixtures/indent_word_online/word/theme/theme1.xml`
- `fixtures/indent_word_online/word/webSettings.xml`

## `fixtures/insert_table/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/insert_table/insert_table.docx`

## `fixtures/instr_links/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/instr_links/instr_links.docx`

## `fixtures/issue554/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/issue554/issue554.docx`

## `fixtures/line_spacing/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/line_spacing/line_spacing.docx`

## `fixtures/link/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/link/[Content_Types].xml`
- `fixtures/link/docProps/app.xml`
- `fixtures/link/docProps/core.xml`
- `fixtures/link/link.docx`
- `fixtures/link/_rels/.rels`
- `fixtures/link/word/document.xml`
- `fixtures/link/word/fontTable.xml`
- `fixtures/link/word/_rels/document.xml.rels`
- `fixtures/link/word/settings.xml`
- `fixtures/link/word/styles.xml`
- `fixtures/link/word/theme/theme1.xml`
- `fixtures/link/word/webSettings.xml`

## `fixtures/lvl_override/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/lvl_override/app.xml`
- `fixtures/lvl_override/[Content_Types].xml`
- `fixtures/lvl_override/core.xml`
- `fixtures/lvl_override/document.xml`
- `fixtures/lvl_override/fontTable.xml`
- `fixtures/lvl_override/numbering.xml`
- `fixtures/lvl_override/override.docx`
- `fixtures/lvl_override/.rels`
- `fixtures/lvl_override/settings.xml`
- `fixtures/lvl_override/styles.xml`
- `fixtures/lvl_override/webSettings.xml`

## `fixtures/multi_paragraph_comment/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/multi_paragraph_comment/multi_paragraph_comment.docx`

## `fixtures/nested_comments/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/nested_comments/[Content_Types].xml`
- `fixtures/nested_comments/docProps/app.xml`
- `fixtures/nested_comments/docProps/core.xml`
- `fixtures/nested_comments/nested_comments.docx`
- `fixtures/nested_comments/_rels/.rels`
- `fixtures/nested_comments/word/commentsExtended.xml`
- `fixtures/nested_comments/word/commentsExtensible.xml`
- `fixtures/nested_comments/word/commentsIds.xml`
- `fixtures/nested_comments/word/comments.xml`
- `fixtures/nested_comments/word/document.xml`
- `fixtures/nested_comments/word/fontTable.xml`
- `fixtures/nested_comments/word/people.xml`
- `fixtures/nested_comments/word/_rels/document.xml.rels`
- `fixtures/nested_comments/word/settings.xml`
- `fixtures/nested_comments/word/styles.xml`
- `fixtures/nested_comments/word/theme/theme1.xml`
- `fixtures/nested_comments/word/webSettings.xml`

## `fixtures/nested_table/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/nested_table/nested_table.docx`

## `fixtures/numbering/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/numbering/[Content_Types].xml`
- `fixtures/numbering/docProps/app.xml`
- `fixtures/numbering/docProps/core.xml`
- `fixtures/numbering/numbering.docx`
- `fixtures/numbering/_rels/.rels`
- `fixtures/numbering/word/document.xml`
- `fixtures/numbering/word/fontTable.xml`
- `fixtures/numbering/word/numbering.xml`
- `fixtures/numbering/word/_rels/document.xml.rels`
- `fixtures/numbering/word/settings.xml`
- `fixtures/numbering/word/styles.xml`

## `fixtures/outline_lvl/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/outline_lvl/outline_lvl.docx`

## `fixtures/page_num_in_header/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/page_num_in_header/page_num_in_header.docx`

## `fixtures/paragraph/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/paragraph/[Content_Types].xml`
- `fixtures/paragraph/docProps/app.xml`
- `fixtures/paragraph/docProps/core.xml`
- `fixtures/paragraph/paragraph.docx`
- `fixtures/paragraph/_rels/.rels`
- `fixtures/paragraph/word/document.xml`
- `fixtures/paragraph/word/fontTable.xml`
- `fixtures/paragraph/word/numbering.xml`
- `fixtures/paragraph/word/_rels/document.xml.rels`
- `fixtures/paragraph/word/settings.xml`
- `fixtures/paragraph/word/styles.xml`

## `fixtures/paragraph_property_change/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/paragraph_property_change/paragraph_property_change.docx`

## `fixtures/ptab/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/ptab/ptab.docx`

## `fixtures/read_hang/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/read_hang/read_hang.docx`

## `fixtures/run_property_change/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/run_property_change/ignore.docx`

## `fixtures/run_props/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/run_props/[Content_Types].xml`
- `fixtures/run_props/docProps/app.xml`
- `fixtures/run_props/docProps/core.xml`
- `fixtures/run_props/_rels/.rels`
- `fixtures/run_props/run_props.docx`
- `fixtures/run_props/word/document.xml`
- `fixtures/run_props/word/fontTable.xml`
- `fixtures/run_props/word/_rels/document.xml.rels`
- `fixtures/run_props/word/settings.xml`
- `fixtures/run_props/word/styles.xml`

## `fixtures/section_current/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/section_current/[Content_Types].xml`
- `fixtures/section_current/docProps/app.xml`
- `fixtures/section_current/docProps/core.xml`
- `fixtures/section_current/_rels/.rels`
- `fixtures/section_current/word/document.xml`
- `fixtures/section_current/word/fontTable.xml`
- `fixtures/section_current/word/_rels/document.xml.rels`
- `fixtures/section_current/word/settings.xml`
- `fixtures/section_current/word/styles.xml`
- `fixtures/section_current/word/theme/theme1.xml`
- `fixtures/section_current/word/webSettings.xml`

## `fixtures/section_even_page/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/section_even_page/[Content_Types].xml`
- `fixtures/section_even_page/docProps/app.xml`
- `fixtures/section_even_page/docProps/core.xml`
- `fixtures/section_even_page/_rels/.rels`
- `fixtures/section_even_page/word/document.xml`
- `fixtures/section_even_page/word/endnotes.xml`
- `fixtures/section_even_page/word/fontTable.xml`
- `fixtures/section_even_page/word/footer1.xml`
- `fixtures/section_even_page/word/footer2.xml`
- `fixtures/section_even_page/word/footer3.xml`
- `fixtures/section_even_page/word/footnotes.xml`
- `fixtures/section_even_page/word/header1.xml`
- `fixtures/section_even_page/word/header2.xml`
- `fixtures/section_even_page/word/header3.xml`
- `fixtures/section_even_page/word/header4.xml`
- `fixtures/section_even_page/word/header5.xml`
- `fixtures/section_even_page/word/_rels/document.xml.rels`
- `fixtures/section_even_page/word/settings.xml`
- `fixtures/section_even_page/word/styles.xml`
- `fixtures/section_even_page/word/theme/theme1.xml`
- `fixtures/section_even_page/word/webSettings.xml`

## `fixtures/section_next_page/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/section_next_page/word/settings.xml`

## `fixtures/section_property_in_ppr/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/section_property_in_ppr/section_property_in_ppr.docx`

## `fixtures/shape/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/shape/shape.docx`

## `fixtures/spacing/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/spacing/[Content_Types].xml`
- `fixtures/spacing/docProps/app.xml`
- `fixtures/spacing/docProps/core.xml`
- `fixtures/spacing/_rels/.rels`
- `fixtures/spacing/word/document.xml`
- `fixtures/spacing/word/fontTable.xml`
- `fixtures/spacing/word/_rels/document.xml.rels`
- `fixtures/spacing/word/settings.xml`
- `fixtures/spacing/word/styles.xml`
- `fixtures/spacing/word/theme/theme1.xml`
- `fixtures/spacing/word/webSettings.xml`

## `fixtures/spec_vanish/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/spec_vanish/spec_vanish.docx`

## `fixtures/strike/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/strike/strike.docx`

## `fixtures/tab_and_break/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/tab_and_break/[Content_Types].xml`
- `fixtures/tab_and_break/docProps/app.xml`
- `fixtures/tab_and_break/docProps/core.xml`
- `fixtures/tab_and_break/_rels/.rels`
- `fixtures/tab_and_break/tab_and_break.docx`
- `fixtures/tab_and_break/word/document.xml`
- `fixtures/tab_and_break/word/fontTable.xml`
- `fixtures/tab_and_break/word/_rels/document.xml.rels`
- `fixtures/tab_and_break/word/settings.xml`
- `fixtures/tab_and_break/word/styles.xml`

## `fixtures/table_border/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_border/[Content_Types].xml`
- `fixtures/table_border/docProps/app.xml`
- `fixtures/table_border/docProps/core.xml`
- `fixtures/table_border/_rels/.rels`
- `fixtures/table_border/table_border.docx`
- `fixtures/table_border/word/document.xml`
- `fixtures/table_border/word/fontTable.xml`
- `fixtures/table_border/word/_rels/document.xml.rels`
- `fixtures/table_border/word/settings.xml`
- `fixtures/table_border/word/styles.xml`
- `fixtures/table_border/word/theme/theme1.xml`
- `fixtures/table_border/word/webSettings.xml`

## `fixtures/table_docx/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_docx/[Content_Types].xml`
- `fixtures/table_docx/docProps/app.xml`
- `fixtures/table_docx/docProps/core.xml`
- `fixtures/table_docx/_rels/.rels`
- `fixtures/table_docx/table.docx`
- `fixtures/table_docx/word/document.xml`
- `fixtures/table_docx/word/footer1.xml`
- `fixtures/table_docx/word/footnotes.xml`
- `fixtures/table_docx/word/header1.xml`
- `fixtures/table_docx/word/numbering.xml`
- `fixtures/table_docx/word/_rels/document.xml.rels`
- `fixtures/table_docx/word/_rels/footer1.xml.rels`
- `fixtures/table_docx/word/_rels/header1.xml.rels`
- `fixtures/table_docx/word/settings.xml`
- `fixtures/table_docx/word/styles.xml`

## `fixtures/table_indent/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_indent/table_indent.docx`

## `fixtures/table_libre_office/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_libre_office/[Content_Types].xml`
- `fixtures/table_libre_office/docProps/app.xml`
- `fixtures/table_libre_office/docProps/core.xml`
- `fixtures/table_libre_office/_rels/.rels`
- `fixtures/table_libre_office/table.docx`
- `fixtures/table_libre_office/word/document.xml`
- `fixtures/table_libre_office/word/fontTable.xml`
- `fixtures/table_libre_office/word/_rels/document.xml.rels`
- `fixtures/table_libre_office/word/settings.xml`
- `fixtures/table_libre_office/word/styles.xml`

## `fixtures/table_merged_libre_office/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_merged_libre_office/[Content_Types].xml`
- `fixtures/table_merged_libre_office/docProps/app.xml`
- `fixtures/table_merged_libre_office/docProps/core.xml`
- `fixtures/table_merged_libre_office/_rels/.rels`
- `fixtures/table_merged_libre_office/table_merged.docx`
- `fixtures/table_merged_libre_office/word/document.xml`
- `fixtures/table_merged_libre_office/word/fontTable.xml`
- `fixtures/table_merged_libre_office/word/_rels/document.xml.rels`
- `fixtures/table_merged_libre_office/word/settings.xml`
- `fixtures/table_merged_libre_office/word/styles.xml`

## `fixtures/table_style/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_style/table_style.docx`

## `fixtures/table_valign/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_valign/[Content_Types].xml`
- `fixtures/table_valign/docProps/app.xml`
- `fixtures/table_valign/docProps/core.xml`
- `fixtures/table_valign/_rels/.rels`
- `fixtures/table_valign/table_valign.docx`
- `fixtures/table_valign/word/document.xml`
- `fixtures/table_valign/word/fontTable.xml`
- `fixtures/table_valign/word/_rels/document.xml.rels`
- `fixtures/table_valign/word/settings.xml`
- `fixtures/table_valign/word/styles.xml`

## `fixtures/table_word_online/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/table_word_online/[Content_Types].xml`
- `fixtures/table_word_online/docProps/app.xml`
- `fixtures/table_word_online/docProps/core.xml`
- `fixtures/table_word_online/_rels/.rels`
- `fixtures/table_word_online/table.docx`
- `fixtures/table_word_online/word/document2.xml`
- `fixtures/table_word_online/word/fontTable.xml`
- `fixtures/table_word_online/word/_rels/document2.xml.rels`
- `fixtures/table_word_online/word/settings.xml`
- `fixtures/table_word_online/word/styles.xml`
- `fixtures/table_word_online/word/theme/theme1.xml`
- `fixtures/table_word_online/word/webSettings.xml`

## `fixtures/textbox/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/textbox/[Content_Types].xml`
- `fixtures/textbox/customXml/item1.xml`
- `fixtures/textbox/customXml/itemProps1.xml`
- `fixtures/textbox/customXml/_rels/item1.xml.rels`
- `fixtures/textbox/docProps/app.xml`
- `fixtures/textbox/docProps/core.xml`
- `fixtures/textbox/_rels/.rels`
- `fixtures/textbox/textbox.docx`
- `fixtures/textbox/word/document.xml`
- `fixtures/textbox/word/fontTable.xml`
- `fixtures/textbox/word/_rels/document.xml.rels`
- `fixtures/textbox/word/settings.xml`
- `fixtures/textbox/word/styles.xml`
- `fixtures/textbox/word/theme/theme1.xml`
- `fixtures/textbox/word/webSettings.xml`

## `fixtures/toc0/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/toc0/toc0.docx`

## `fixtures/toc1/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/toc1/toc1.docx`

## `fixtures/tr2bl/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/tr2bl/tr2bl.docx`

## `fixtures/vert_align/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/vert_align/vert_align.docx`

## `fixtures/without_numid/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/without_numid/without_numid.docx`

## `fixtures/word_default/`

Data: a .docx fixture the reader tests open, kept whole and unpacked into its parts.

- `fixtures/word_default/[Content_Types].xml`
- `fixtures/word_default/docProps/app.xml`
- `fixtures/word_default/docProps/core.xml`
- `fixtures/word_default/_rels/.rels`
- `fixtures/word_default/word_default.docx`
- `fixtures/word_default/word/document.xml`
- `fixtures/word_default/word/fontTable.xml`
- `fixtures/word_default/word/numbering.xml`
- `fixtures/word_default/word/_rels/document.xml.rels`
- `fixtures/word_default/word/settings.xml`
- `fixtures/word_default/word/styles.xml`
- `fixtures/word_default/word/theme/theme1.xml`
- `fixtures/word_default/word/webSettings.xml`
