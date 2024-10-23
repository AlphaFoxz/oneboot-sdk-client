export {}

declare global {
  function isNever(v: never): void
  type FileInfo = {
    isFile: boolean
    isFolder: boolean
    path: string
    content: string | undefined
  }
  type NonEmptyArray<T> = [T, ...T[]]
  enum $JavaSyntax {
    package_declaration = 'package_declaration',
    import_declaration = 'import_declaration',
    scoped_identifier = 'scoped_identifier',
    class_declaration = 'class_declaration',
    interface_declaration = 'interface_declaration',
    enum_declaration = 'enum_declaration',
    record_declaration = 'record_declaration',
    generic_type = 'generic_type',
    type_identifier = 'type_identifier',
    identifier = 'identifier',
    formal_parameters = 'formal_parameters',
    formal_parameter = 'formal_parameter',
  }
}
