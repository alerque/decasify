#import "../typst/decasify.typ": sentencecase, titlecase

#set text(lang: "en")

#let sample = [this sample *has bold* and _italic #underline[underlined]_ bits.]

#table(columns: (2cm, 9cm))[Source][#sample]

= Sentencecase

#table(columns: (2cm, 9cm))[Actual][
  #sentencecase[#sample]
][Expected][
  This sample *has bold* and _italic #underline[underlined]_ bits.
]

= Titlecase

#table(columns: (2cm, 9cm))[Actual][
  #titlecase[#sample]
][Expected][
  This Sample *Has Bold* and _Italic #underline[Underlined]_ Bits.
]
