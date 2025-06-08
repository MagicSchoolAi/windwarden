# Custom Sorting Demo

This demonstrates WindWarden's custom sorting functionality.

## Example Configuration

```json
{
  "sortOrder": "custom",
  "customOrder": ["effects", "transitions", "transforms", "backgrounds", "typography", "borders", "spacing", "sizing", "flexbox-grid", "layout"]
}
```

## Before (Official Tailwind Order)
```tsx
<div className="filter-blur backdrop-blur-sm border-4 border-dashed border-blue-500 p-8 m-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white text-xl font-bold shadow-2xl rounded-lg flex flex-col items-center justify-center gap-4 w-full h-screen transform rotate-1 hover:rotate-0 transition-all duration-300 ease-in-out">
```

## After (Custom Order - Effects First)
```tsx  
<div className="shadow-2xl duration-300 ease-in-out transition-all hover:rotate-0 rotate-1 transform bg-gradient-to-r from-blue-500 to-purple-600 font-bold text-white text-xl border-4 border-blue-500 border-dashed rounded-lg m-4 p-8 h-screen w-full flex flex-col gap-4 items-center justify-center backdrop-blur-sm filter-blur">
```

## Available Categories

layout, flexbox-grid, spacing, sizing, typography, backgrounds, borders, effects, filters, tables, transitions, transforms, interactivity, svg, accessibility, unknown

## Usage

1. Create config: `windwarden config init`
2. Edit `.windwarden.json` and set `"sortOrder": "custom"`
3. Define your preferred order in `"customOrder": [...]`
4. Run: `windwarden format src/`

The custom sorting allows teams to prioritize the CSS categories that matter most to their workflow!