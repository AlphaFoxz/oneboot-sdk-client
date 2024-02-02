import { editor } from 'monaco-editor'

export namespace component_options {
  export class TreeItem {
    title: Function | string = ''
    key = ''
    isLeaf = false
    selectable = true
    children: Array<TreeItem> | undefined = undefined
    _data: {
      [k: string]: any
    } = {}
  }
  /* 主题 */
  export declare type Theme = editor.BuiltinTheme
  export declare type OtherOptions = editor.IEditorOptions & editor.IGlobalEditorOptions
}

export namespace component_dto {
  export interface Namespace {
    namespace_lang: any
    namespace_value: any
  }
  export interface Include {}
  export interface Struct {}
  export interface Enum {}
  export interface Service {}

  export type CheckResult = {
    success: boolean
    location: {
      Pos: number | null
      span: Array<number> | null
    }
    line_col: {
      PosLine: Array<number> | null
      SpanLine: Array<Array<number>> | null
    }
    message: string | null
  }
}

export declare namespace monaco_editor {
  export type IStandaloneCodeEditor = editor.IStandaloneCodeEditor
  export type IStandaloneEditorConstructionOptions = editor.IStandaloneEditorConstructionOptions
  export type IEditorOverrideServices = editor.IEditorOverrideServices
  export type IEditorOptions = editor.IEditorOptions
  export type IGlobalEditorOptions = editor.IGlobalEditorOptions
}

export enum GenTypeEnum {
  GEN_JAVA_SERVER_CODE = 'generateJavaServerCode',
  GEN_TS_CLIENT_CODE = 'generateTsClientCode',
  GEN_RUST_CLIENT_CODE = 'generateRustClientCode',
  GEN_DB_SQL = 'generateDbSql',
}
