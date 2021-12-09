(struct_specifier
  (attribute_declaration
    (attribute) @attribute)
  name: (_) @name
  body: (field_declaration_list
	(field_declaration
	  type: (_) @type
	  declarator: (_) @decl
	  default_value: (_)? @default)))

(class_specifier
  (attribute_declaration
    (attribute) @attribute)
  name: (_) @name
  body: (field_declaration_list
	(field_declaration
	  type: (_) @type
	  declarator: (_) @decl
	  default_value: (_)? @default)))
