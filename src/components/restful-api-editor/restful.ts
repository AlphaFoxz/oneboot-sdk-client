export function registerRestful(monaco: { languages: any }) {
  const languages = monaco.languages
  const keywords = [
    /* 关键字 */
    'include',
    'namespace',
    'service',
    'enum',
    'struct',
    'required',
    'optional',
    /* 内置类型 */
    'i16',
    'i32',
    'i64',
    'bool',
    'byte',
    'map',
    'list',
    'set',
  ]

  languages.register({ id: 'thrift', extensions: ['.thrift'] })
  /*
  内置类型
  identifier         entity           constructor
  operators          tag              namespace
  keyword            info-token       type
  string             warn-token       predefined
  string.escape      error-token      invalid
  comment            debug-token
  comment.doc        regexp
  constant           attribute

  delimiter .[curly,square,parenthesis,angle,array,bracket]
  number    .[hex,octal,binary,float]
  variable  .[name,value]
  meta      .[content]
  */
  languages.setMonarchTokensProvider('thrift', {
    keyword: keywords,
    tokenizer: {
      root: [
        [/\bnamespace\b/, { token: 'keyword', next: '@namespace_body' }],

        [
          /[a-zA-Z][\w$]*/,
          {
            cases: {
              '@keyword': 'keyword',
              '@default': 'type',
            },
          },
        ],
        [/[0-9]+\s*:/, 'identifier'],
        [/\"[^\"]*\"/, 'string'],

        [/\/\/[\s]*@[^\(]+/, { token: 'tag', next: '@annoValue' }],
        [/\/\/.*$/, 'comment'],
        [/\/\*/, { token: 'comment.doc', next: '@commentDoc' }],
      ],
      annoValue: [
        [/\(|,/, 'delemiter'],
        [/\)/, { token: 'delemiter', next: '@pop' }],
        [/[,]/, 'delemiter'],
        [/[^(\(|\)|,)]*/, 'string'],
      ],
      commentDoc: [
        [/\*\//, { token: 'comment.doc', next: '@pop' }],
        [/[^*/]+/, 'comment.doc'],
        [/[*/]/, 'comment.doc'],
      ],
      namespace_body: [
        [/[a-zA-Z_\.]+\s*$/, { token: 'namespace', next: '@pop' }],
        [/\b[a-zA-Z]+\b/, 'variable'],
      ],
      service_header: [
        [/}/, '@pop'],
        [/\/\/.*$/, 'comment'],
        [/\/\*/, { token: 'comment.doc', next: '@commentDoc' }],

        [/\b[a-zA-Z_][\w$]*/, 'type'],
      ],
      service_body: [],
    },
  })

  languages.register({
    id: 'restful',
    aliases: ['rest'],
    extensions: ['.rest', '.restful'],
    mimetypes: ['text/x-restful', 'text/x-restful-source'],
  })
  languages.setLanguageConfiguration('restful', {
    comments: {
      lineComment: '//',
      blockComment: ['/*', '*/'],
    },
    brackets: [
      ['(', ')'],
      ['{', '}'],
      ['[', ']'],
    ],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
    surroundingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
      { open: '<', close: '>' },
    ],
  })

  languages.setMonarchTokensProvider('restful', {
    keyword: [
      'namespace',
      'import',
      'interface',
      'class',
      'enum',
      'boolean',
      'byte',
      'binary',
      'short',
      'int',
      'long',
      'double',
      'i16',
      'i32',
      'i64',
      'string',
      'list',
      'set',
      'map',
      'void',
      'required',
      'optional',
    ],
    tokenizer: {
      root: [
        [/\bnamespace\b/, { token: 'keyword', next: '@namespace_body' }],

        [
          /[a-zA-Z][\w$]*/,
          {
            cases: {
              '@keyword': 'keyword',
              '@default': 'type',
            },
          },
        ],
        [/[0-9]+\s*:/, 'identifier'],
        [/\"[^\"]*\"/, 'string'],

        [/\/\/[\s]*@[^\(]+/, { token: 'tag', next: '@annoValue' }],
        [/\/\/.*$/, 'comment'],
        [/\/\*/, { token: 'comment.doc', next: '@commentDoc' }],
      ],
      annoValue: [
        [/\(|,/, 'delemiter'],
        [/\)/, { token: 'delemiter', next: '@pop' }],
        [/[,]/, 'delemiter'],
        [/[^(\(|\)|,)]*/, 'string'],
      ],
      commentDoc: [
        [/\*\//, { token: 'comment.doc', next: '@pop' }],
        [/[^*/]+/, 'comment.doc'],
        [/[*/]/, 'comment.doc'],
      ],
      namespace_body: [
        [/[a-zA-Z_\.]+\s*$/, { token: 'namespace', next: '@pop' }],
        [/\b[a-zA-Z]+\b/, 'variable'],
      ],
      service_header: [
        [/}/, '@pop'],
        [/\/\/.*$/, 'comment'],
        [/\/\*/, { token: 'comment.doc', next: '@commentDoc' }],

        [/\b[a-zA-Z_][\w$]*/, 'type'],
      ],
      service_body: [],
    },
  })
}
