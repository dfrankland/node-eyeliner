export default (options: {
  html: string,
  css?: string | null,
  options?: {
    applyTableElementAttributes?: boolean | null,
    applyHeightAttributes?: boolean | null,
    applyStyleTags?: boolean | null,
    applyWidthAttributes?: boolean | null,
    insertPreservedCss?: string[] | null,
    preserveFontFaces?: boolean | null,
    preserveImportant?: boolean | null,
    preserveMediaQueries?: boolean | null,
    removeStyleTags?: boolean | null,
  } | null,
  settings?: {
    widthElements?: string[] | null,
    heightElements?: string[] | null,
    styleToAttribute?: {
      [key: string]: string
    } | null,
    tableElements: string[] | null,
    nonVisualElements: string[] | null,
    excludedProperties: string[] | null,
  } | null,
}) => string;

export const setPrefs = (prefsPath: string) => void;
