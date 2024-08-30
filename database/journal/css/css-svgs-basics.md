# SVG

SVG (Scalable Vector Graphics) provides several basic shapes that you can use to create various graphics. Here are the main shapes available in SVG:

1. **Rectangle (`<rect>`)**:

   ```html
   <svg width="100" height="100">
     <rect x="10" y="10" width="80" height="80" fill="blue" />
   </svg>
   ```

2. **Circle (`<circle>`)**:

   ```html
   <svg width="100" height="100">
     <circle cx="50" cy="50" r="40" fill="red" />
   </svg>
   ```

3. **Ellipse (`<ellipse>`)**:

   ```html
   <svg width="100" height="100">
     <ellipse cx="50" cy="50" rx="40" ry="20" fill="green" />
   </svg>
   ```

4. **Line (`<line>`)**:

   ```html
   <svg width="100" height="100">
     <line x1="10" y1="10" x2="90" y2="90" stroke="black" stroke-width="2" />
   </svg>
   ```

5. **Polygon (`<polygon>`)**:

   ```html
   <svg width="100" height="100">
     <polygon points="50,10 90,90 10,90" fill="purple" />
   </svg>
   ```

6. **Polyline (`<polyline>`)**:

   ```html
   <svg width="100" height="100">
     <polyline
       points="10,10 50,50 90,10"
       fill="none"
       stroke="orange"
       stroke-width="2"
     />
   </svg>
   ```

7. **Path (`<path>`)**:

   ```html
   <svg width="100" height="100">
     <path
       d="M10 80 Q 95 10 180 80"
       stroke="brown"
       fill="none"
       stroke-width="2"
     />
   </svg>
   ```

## Explanation of Attributes

- **`<rect>`**: Defines a rectangle.
  - `x`, `y`: Coordinates of the upper-left corner.
  - `width`, `height`: Dimensions of the rectangle.
- **`<circle>`**: Defines a circle.
  - `cx`, `cy`: Center coordinates.
  - `r`: Radius.
- **`<ellipse>`**: Defines an ellipse.
  - `cx`, `cy`: Center coordinates.
  - `rx`, `ry`: Radii for the x and y axes.
- **`<line>`**: Defines a line.
  - `x1`, `y1`: Starting point coordinates.
  - `x2`, `y2`: Ending point coordinates.
- **`<polygon>`**: Defines a closed shape consisting of a series of connected points.
  - `points`: List of points in the format `x1,y1 x2,y2 x3,y3 ...`.
- **`<polyline>`**: Similar to `<polygon>`, but does not automatically close the shape.

  - `points`: List of points in the format `x1,y1 x2,y2 x3,y3 ...`.

- **`<path>`**: Defines a complex shape using a series of commands.
  - `d`: Contains the path data. Commands include `M` (move to), `L` (line to), `H` (horizontal line to), `V` (vertical line to), `C` (cubic Bezier curve), `S` (smooth cubic Bezier curve), `Q` (quadratic Bezier curve), `T` (smooth quadratic Bezier curve), `A` (elliptical Arc), and `Z` (close path).

These elements provide the building blocks for creating complex SVG graphics and animations.

Each SVG shape has a variety of attributes that define its appearance and behavior. Here's a detailed look at the main attributes available for each type of SVG shape:

### Rectangle (`<rect>`)

- `x`: The x-axis coordinate of the rectangle's starting point.
- `y`: The y-axis coordinate of the rectangle's starting point.
- `width`: The width of the rectangle.
- `height`: The height of the rectangle.
- `rx`: The x-axis radius of the rectangle's rounded corners.
- `ry`: The y-axis radius of the rectangle's rounded corners.
- `fill`: The fill color of the rectangle.
- `stroke`: The color of the rectangle's outline.
- `stroke-width`: The width of the rectangle's outline.

### Circle (`<circle>`)

- `cx`: The x-axis coordinate of the center of the circle.
- `cy`: The y-axis coordinate of the center of the circle.
- `r`: The radius of the circle.
- `fill`: The fill color of the circle.
- `stroke`: The color of the circle's outline.
- `stroke-width`: The width of the circle's outline.

### Ellipse (`<ellipse>`)

- `cx`: The x-axis coordinate of the center of the ellipse.
- `cy`: The y-axis coordinate of the center of the ellipse.
- `rx`: The x-axis radius of the ellipse.
- `ry`: The y-axis radius of the ellipse.
- `fill`: The fill color of the ellipse.
- `stroke`: The color of the ellipse's outline.
- `stroke-width`: The width of the ellipse's outline.

### Line (`<line>`)

- `x1`: The x-axis coordinate of the starting point of the line.
- `y1`: The y-axis coordinate of the starting point of the line.
- `x2`: The x-axis coordinate of the ending point of the line.
- `y2`: The y-axis coordinate of the ending point of the line.
- `stroke`: The color of the line.
- `stroke-width`: The width of the line.

### Polygon (`<polygon>`)

- `points`: A list of points that define the vertices of the polygon.
- `fill`: The fill color of the polygon.
- `stroke`: The color of the polygon's outline.
- `stroke-width`: The width of the polygon's outline.

### Polyline (`<polyline>`)

- `points`: A list of points that define the vertices of the polyline.
- `fill`: The fill color of the polyline.
- `stroke`: The color of the polyline.
- `stroke-width`: The width of the polyline.

### Path (`<path>`)

- `d`: A string containing a series of commands that define the path's shape.
- `fill`: The fill color of the path.
- `stroke`: The color of the path's outline.
- `stroke-width`: The width of the path's outline.

### Common Attributes

In addition to the specific attributes listed above, there are several common attributes that can be used with any SVG shape:

- `opacity`: The opacity of the shape (0 to 1).
- `transform`: Transformations such as `translate`, `rotate`, `scale`, and `skew` to alter the shape's position, size, and orientation.
- `fill-opacity`: The opacity of the shape's fill color.
- `stroke-opacity`: The opacity of the shape's stroke color.
- `stroke-dasharray`: A pattern of dashes and gaps used to outline the shape.
- `stroke-dashoffset`: The starting point of the dash pattern.
- `stroke-linecap`: The style of the ends of the lines (`butt`, `round`, `square`).
- `stroke-linejoin`: The style of the joins between connected lines (`miter`, `round`, `bevel`).

### Examples

Here are examples of each shape with a few attributes:

#### Rectangle

```html
<rect
  x="10"
  y="10"
  width="80"
  height="50"
  rx="10"
  ry="10"
  fill="blue"
  stroke="black"
  stroke-width="2"
/>
```

#### Circle

```html
<circle cx="50" cy="50" r="40" fill="red" stroke="black" stroke-width="2" />
```

#### Ellipse

```html
<ellipse
  cx="50"
  cy="50"
  rx="40"
  ry="20"
  fill="green"
  stroke="black"
  stroke-width="2"
/>
```

#### Line

```html
<line x1="10" y1="10" x2="90" y2="90" stroke="black" stroke-width="2" />
```

#### Polygon

```html
<polygon
  points="50,10 90,90 10,90"
  fill="purple"
  stroke="black"
  stroke-width="2"
/>
```

#### Polyline

```html
<polyline
  points="10,10 50,50 90,10"
  fill="none"
  stroke="orange"
  stroke-width="2"
/>
```

#### Path

```html
<path d="M10 80 Q 95 10 180 80" stroke="brown" fill="none" stroke-width="2" />
```

These attributes and shapes allow for a wide variety of graphics and animations to be created using SVG.
