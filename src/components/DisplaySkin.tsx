import { createMemo, createResource, useContext } from 'solid-js';

import { SkinWithSpray } from './SkinSelect';
import { SkinOptions, SpingenContext } from '../spingen';

export interface DisplaySkinProps {
  skin: () => SkinWithSpray | undefined;
  options: () => SkinOptions;
}

const DisplaySkin = (props: DisplaySkinProps) => {
  const spingen = useContext(SpingenContext);

  const [imgBlob] = createResource(
    () => {
      const innerSkin = props.skin();
      if (innerSkin) {
        const {spray, setSpray: __, ...skin} = innerSkin;
        return {
          spray: spray() ?? null,
          options: props.options(),
          skin,
        };
      } else {
        return null;
      }
    },
    async (data) => {
      const url = await spingen?.createSkinAnimation(data.skin, data.spray, data.options);
      return url;
    },
  )

  const imgSrc = createMemo<string | undefined>((oldSrc) => {
    if (oldSrc) {
      URL.revokeObjectURL(oldSrc);
    }
    return imgBlob();
  });

  return (
    <img src={imgSrc()} alt="skin image"/>
  )
};

export default DisplaySkin;
