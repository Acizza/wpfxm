import React, { CSSProperties, useMemo, useRef, useState } from "react";

interface GenericListProps<T> {
  items: T[];
  display?(item: T): string;
  highlight?(item: T): boolean;
  onItemSelected?(item: T, selected: boolean): void;
}

function GenericList<T>(props: GenericListProps<T>): JSX.Element {
  const [primarySelection, setPrimarySelection] = useSelection();

  function itemClicked(item: T, index: number) {
    const isSelected = setPrimarySelection(index);
    props.onItemSelected?.(item, isSelected);
  }

  const items = useMemo(() => {
    const isHighlighted = props.highlight
      ? (item: T) => props.highlight!(item)
      : () => false;

    return props.items.map((item, i) => (
      <ScrollingItemEntry
        key={i}
        item={item}
        selected={i === primarySelection}
        highlighted={isHighlighted(item)}
        display={props.display}
        onClick={() => itemClicked(item, i)}
      />
    ));
  }, [props.items, props.display, props.highlight, primarySelection]);

  return <ul className="generic-list">{items}</ul>;
}

type SetSelected = (value?: number) => boolean;

function useSelection(initial?: number): [number | undefined, SetSelected] {
  const [state, setState] = useState(initial);

  function set(value?: number): boolean {
    const newValue = value === state ? undefined : value;
    setState(newValue);

    return newValue !== undefined;
  }

  return [state, set];
}

interface ItemEntryProps<T> {
  item: T;
  selected: boolean;
  highlighted: boolean;
  display?(item: T): string;
  onClick?(event: React.MouseEvent): void;
}

function ScrollingItemEntry<T>(props: ItemEntryProps<T>): JSX.Element {
  const [style, setStyle] = useState<CSSProperties | undefined>(undefined);
  const textRef = useRef<HTMLSpanElement>(null);

  const display = props.display || ((item) => item);
  let classes = "generic-list list-item";

  if (props.selected) classes += " item-selected";
  if (props.highlighted) classes += " item-highlighted";

  function onMouseOver() {
    const elem = textRef.current;

    if (!elem) return;

    const overflow = elem.scrollWidth - elem.offsetWidth;

    if (overflow < 1) return;

    setStyle({
      transform: `translateX(-${overflow}px)`,
      transitionDuration: `${Math.max(overflow * 10, 500)}ms`,
    });
  }

  function onMouseLeave() {
    setStyle(undefined);
  }

  return (
    <li
      className={classes}
      onClick={props.onClick}
      onMouseOver={onMouseOver}
      onMouseLeave={onMouseLeave}
    >
      <span className="text" ref={textRef} style={style}>
        {display(props.item)}
      </span>
    </li>
  );
}

export default GenericList;
