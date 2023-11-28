import { invoke } from '@tauri-apps/api/primitives'
import * as types from './define'
type IStandaloneCodeEditor = types.monaco_editor.IStandaloneCodeEditor

export async function checkErr(monaco: any, codeStr: string, editorInst?: IStandaloneCodeEditor): Promise<boolean> {
  const result = (await invoke('check_restful_code_err', {
    code: codeStr,
  })) as types.component_dto.CheckResult
  monaco.editor.removeAllMarkers('restful')
  if (result.success) {
    return true
  }
  let startLineNumber = 1
  let startColumn = 1
  let endLineNumber = 1
  let endColumn = 1
  if (result.line_col?.PosLine) {
    const p = result.line_col?.PosLine
    startLineNumber = p[0]
    endLineNumber = p[0]
    startColumn = p[1]
    endColumn = p[1]
  } else if (result.line_col?.SpanLine) {
    const p = result.line_col?.SpanLine
    startLineNumber = p[0][0]
    endLineNumber = p[1][0]
    startColumn = p[0][1]
    endColumn = p[1][1]
  }
  if (editorInst) {
    monaco.editor.setModelMarkers(editorInst.getModel()!, 'restful', [
      {
        startLineNumber,
        startColumn,
        endLineNumber,
        endColumn,
        severity: monaco.MarkerSeverity.Error,
        message: result.message!,
      },
    ])
  }
  return false
}
