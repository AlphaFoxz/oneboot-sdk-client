type OsType = 'Windows Phone' | 'Android' | 'iOS' | 'macOS' | 'Windows' | 'Linux' | 'Unknown'

function os(): OsType {
  const userAgent = navigator.userAgent || navigator.vendor || (window as any).opera

  if (/windows phone/i.test(userAgent)) {
    return 'Windows Phone'
  }

  if (/android/i.test(userAgent)) {
    return 'Android'
  }

  if (/iPad|iPhone|iPod/.test(userAgent) && !(window as any).MSStream) {
    return 'iOS'
  }

  if (/Macintosh/i.test(userAgent)) {
    return 'macOS'
  }

  if (/Windows/i.test(userAgent)) {
    return 'Windows'
  }

  if (/Linux/i.test(userAgent)) {
    return 'Linux'
  }

  return 'Unknown'
}

function sep(): string {
  if (os() === 'Windows') {
    return '\\'
  }
  return '/'
}

export default {
  sep: sep(),
  join(...str: string[]): string {
    return str.join(sep())
  },
}
