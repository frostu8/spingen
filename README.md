[`spin.ringrace.rs`](https://spin.ringrace.rs/) is a tool to generate
animations from PK3s and WADs. It has an obvious use for mod developers wanting
to show off their creations in public spaces, but it can also be used by
end-users to preview addons from various angles.

Click at the top left to add your PK3, and the website will load it and compile
your characters to browse. You can also choose alternative sprays.

![controls](https://raw.githubusercontent.com/frostu8/spingen/refs/heads/main/docs/controls.png)

1. The sprite. Racers will have many, many animation sprites for all the
different states and motions they can be in. Try "Spinout," "Dead," and
"Finish Signpost (working designs)" for some fun ones you might not get to
truly appreciate often in races.
2. The animation frame. In game, racers will cycle between these (except for
the Wanted sprites) to give the illusion of a raging engine ready to race. In
`spin.ringrace.rs`, you can view each frame individually.
3. The scale of the sprite. When downloaded, the sprite will come in the
dimension it comes as (for most racers, this is 96x96). You can use this to
upscale the sprite for better display on browsers and social media.
4. "Use Preferred Spray" switches the chosen spray back to the racer's
preferred color.
5. Some information about the racer.

### Why are my animated images appearing blurry on Discord/MB/social media?
<img
  src="https://raw.githubusercontent.com/frostu8/spingen/refs/heads/main/docs/sakura.gif"
  alt="sakura spinning"
  width="384"
  height="384"
/>

You may need to **upscale** your images before downloading them. See control
(3) above. A good balance between image size and clarity is "4x". If you think
you need it, the upscaler goes up to "8x", but your browser might not like it.

`spin.ringrace.rs` will always show racer sprites in crisp quality, even if the
sprites are only in "1x", to save your computer some CPU power.

### My custom colors are written in SOC, and are not showing up!
`spin.ringrace.rs` has first-class support for colors written in SOC; what you
see is what you get. Make sure your colors are defined in a SOC file, under the
`soc/` folder, like this:

```soc
# Colors transpiled from <https://mb.srb2.org/addons/a-r-k-pack.8145/>
# Used as the testing baseline for SOC colors. Go check it out!
Freeslot
SKINCOLOR_MAIZE
SKINCOLOR_JASMINE

Skincolor SKINCOLOR_MAIZE
Name = Maize
ramp = 82,73,74,75,66,66,67,68,105,106,107,108,109,110,111,31
invcolor = SKINCOLOR_HANDHELD
invshade = 7
chatcolor = V_YELLOWMAP
accessible = true

Skincolor SKINCOLOR_JASMINE
name = Jasmine
ramp = 2,5,8,10,12,16,18,94,107,108,108,109,109,110,110,111
invcolor = SKINCOLOR_PISTACHIO
invshade = 7
chatcolor = V_GREENMAP
accessible = true
```

If you need more help, check out the
[KKD Discord](https://www.kartkrew.org/discord/). If you think this is a bug,
shoot a bug report @
[the github repository](https://github.com/frostu8/spingen).

### My custom colors are written in Lua, and are not showing up!
*Lua is hard*.

For the sanity of contributors, `spin.ringrace.rs` only supports a very
specific method of defining colors. These are perfectly O.K. methods of
defining colors in a way the website will pick up:

```lua
--[[
Colors ~~transpiled~~ stolen from <https://mb.srb2.org/addons/a-r-k-pack.8145/>
Go check it out!
--]]
-- perfectly ok!
skincolors[SKINCOLOR_FAMI] = {
  name = "Fami",
  ramp = {80,82,83,84,85,86,246,41,63,44,45,71,46,28,29,30},
  invcolor = SKINCOLOR_DAWN,
  invshade = 7,
  chatcolor = V_REDMAP,
  accessible = true
}

-- also ok! Lua does not care about whitespace and the website won't either.
skincolors[SKINCOLOR_ASIMOV] = {
  name = "Asimov", ramp = {0,1,3,5,6,8,9,134,135,148,149,137,26,27,28,29},
  invcolor = SKINCOLOR_PERIWINKLE, invshade = 7,
  chatcolor = V_BLUEMAP, accessible = true
}
```

It currently does not support these syntaxes, even though these are perfectly
valid ways of defining colors:

```lua
-- nope
skincolors[SKINCOLOR_FAMI] = {
  "Fami",
  {80,82,83,84,85,86,246,41,63,44,45,71,46,28,29,30},
  SKINCOLOR_DAWN,
  7,
  V_REDMAP,
  true
}

-- sorry, not this either
skincolors[SKINCOLOR_ASIMOV].name = "Asimov"
skincolors[SKINCOLOR_ASIMOV].ramp = {0,1,3,5,6,8,9,134,135,148,149,137,26,27,28,29}
skincolors[SKINCOLOR_ASIMOV].invcolor = SKINCOLOR_PERIWINKLE
skincolors[SKINCOLOR_ASIMOV].invshade = 7
skincolors[SKINCOLOR_ASIMOV].chatcolor = V_BLUEMAP
skincolors[SKINCOLOR_ASIMOV].accessible = true
```

And it will **never** support this syntax, because this is horrifying and
nothing short of "running a Lua script" can fix this, even though this is a
valid color definition.

```lua
local function GetName()
  return "Asi" .. "mov"
end

local function GetRamp()
  return {0,1,3,5,6,8,9,134,135,148,149,137,26,27,28,29}
end

-- I will be sad if you do this.
skincolors[SKINCOLOR_ASIMOV].name = GetName()
skincolors[SKINCOLOR_ASIMOV].ramp = GetRamp()
skincolors[SKINCOLOR_ASIMOV].invcolor = SKINCOLOR_PERIWINKLE
skincolors[SKINCOLOR_ASIMOV].invshade = 7
skincolors[SKINCOLOR_ASIMOV].chatcolor = V_BLUEMAP
skincolors[SKINCOLOR_ASIMOV].accessible = not not not not not not true
```

