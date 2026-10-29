/**
 * The icons flint components need, as raw SVG for `<svg source={...}>`.
 *
 * Path data is Lucide (ISC). GPUI paints the markup as one shape, so every
 * icon here is monochrome and takes its colour from the element's `color`.
 */

const wrap = (paths: string, fill = false) =>
  `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="${fill ? "currentColor" : "none"}" stroke="#000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">${paths}</svg>`

export const ICONS = {
  check: wrap('<path d="M20 6 9 17l-5-5"/>'),
  chevronDown: wrap('<path d="m6 9 6 6 6-6"/>'),
  chevronUp: wrap('<path d="m18 15-6-6-6 6"/>'),
  chevronRight: wrap('<path d="m9 18 6-6-6-6"/>'),
  chevronLeft: wrap('<path d="m15 18-6-6 6-6"/>'),
  chevronsUpDown: wrap('<path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>'),
  x: wrap('<path d="M18 6 6 18"/><path d="m6 6 12 12"/>'),
  search: wrap('<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>'),
  plus: wrap('<path d="M5 12h14"/><path d="M12 5v14"/>'),
  minus: wrap('<path d="M5 12h14"/>'),
  square: wrap('<rect width="14" height="14" x="5" y="5" rx="2"/>'),
  alert: wrap('<circle cx="12" cy="12" r="10"/><path d="M12 8v4"/><path d="M12 16h.01"/>'),
  info: wrap('<circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/>'),
  circle: wrap('<circle cx="12" cy="12" r="10"/>'),
  dot: wrap('<circle cx="12" cy="12" r="4"/>', true),
  loader: wrap('<path d="M21 12a9 9 0 1 1-6.219-8.56"/>'),
  panelLeft: wrap(
    '<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/>'
  ),
  sparkle: wrap(
    '<path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z"/>'
  ),
  sun: wrap(
    '<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>'
  ),
  moon: wrap('<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>'),
  users: wrap('<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>'),
  mail: wrap('<rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>'),
  terminal: wrap('<path d="m4 17 6-6-6-6"/><path d="M12 19h8"/>'),
  home: wrap('<path d="m3 10 9-7 9 7v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z"/><path d="M9 22V12h6v10"/>'),
  layers: wrap('<path d="m12 2 9 5-9 5-9-5 9-5Z"/><path d="m3 12 9 5 9-5"/><path d="m3 17 9 5 9-5"/>'),
  sliders: wrap('<path d="M4 21v-7"/><path d="M4 10V3"/><path d="M12 21v-9"/><path d="M12 8V3"/><path d="M20 21v-5"/><path d="M20 12V3"/><path d="M2 14h4"/><path d="M10 8h4"/><path d="M18 16h4"/>'),
  play: wrap('<path d="m7 4 12 8-12 8Z"/>'),
  server: wrap('<rect width="20" height="8" x="2" y="2" rx="2"/><rect width="20" height="8" x="2" y="14" rx="2"/><path d="M6 6h.01"/><path d="M6 18h.01"/>'),
  folder: wrap('<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>'),
  download: wrap('<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/>'),
  hardDrive: wrap('<path d="M22 12H2"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11Z"/><path d="M6 16h.01"/><path d="M10 16h.01"/>'),
  arrowRight: wrap('<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>'),
  arrowUpRight: wrap('<path d="M7 7h10v10"/><path d="M7 17 17 7"/>'),
  globe: wrap('<circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/>'),
  refresh: wrap('<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/>'),
  box: wrap('<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/>'),
  power: wrap('<path d="M12 2v10"/><path d="M18.4 6.6a9 9 0 1 1-12.77.04"/>'),
} as const

export type IconName = keyof typeof ICONS
export const ICON_NAMES = Object.keys(ICONS) as IconName[]
