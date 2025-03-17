import { Spray } from '../types.ts';
import { For } from 'solid-js';

export interface SpraySelectProps {
  sprays: () => Spray[];
};

interface SpraySelectOptionProps {
  spray: Spray;
  selected: boolean;
}

const SpraySelectOption = (props: SpraySelectOptionProps) => {
  const buttonClass = props.selected ? "spray-button selected" : "spray-button";

  return (
    <button class={buttonClass}>
      { props.spray.name }
    </button>
  );
}

const SpraySelect = (props: SpraySelectProps) => {
  return (
    <div class="spray-select">
      <For each={props.sprays()}>
        {(item, _index) => (
          <SpraySelectOption spray={item} selected={false}/>
        )}
      </For>
    </div>
  );
};

export default SpraySelect;
