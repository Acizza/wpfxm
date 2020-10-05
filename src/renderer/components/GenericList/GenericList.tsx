import React, { useMemo, useState } from "react";

interface GenericListProps<T> {
  items: T[];
  display?(item: T): string;
  onItemSelected?(item: T, selected: boolean): void;
}

function GenericList<T>(props: GenericListProps<T>): JSX.Element {
  const [selected, setSelected] = useSelection();

  function itemClicked(item: T, index: number) {
    const isSelected: boolean = setSelected(index);
    props.onItemSelected?.(item, isSelected);
  }

  const items = useMemo(
    () =>
      props.items.map((item, i) => (
        <ItemEntry
          key={i}
          item={item}
          selected={i === selected}
          display={props.display}
          onClick={() => itemClicked(item, i)}
        />
      )),
    [props.items, props.display, selected]
  );

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
  display?(item: T): string;
  onClick?(event: React.MouseEvent): void;
}

function ItemEntry<T>(props: ItemEntryProps<T>): JSX.Element {
  const display = props.display || ((item) => item);

  let classes = "generic-list list-item";

  if (props.selected) classes += " item-selected";

  return (
    <li className={classes} onClick={props.onClick}>
      {display(props.item)}
    </li>
  );
}

export default GenericList;
