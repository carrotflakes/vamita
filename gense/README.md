# gense

シンプルな効果音生成ライブラリ

レシピから効果音を生成します。


オシレータ: sin, saw, square, triangle
ノイズ: whitenoise
ADSRエンベロープ
フィルタ: ローパス, ハイパス, バンドパス
LFO

エフェクト: delay, distortion, reverb, chorus, bitcrusher

## recipe format

```yaml
oscillator:
  type: sine
  frequency: 440
  envelope:
    type: adsr
    adsr: [0.1, 0.2, 0.8, 0.5]
```
