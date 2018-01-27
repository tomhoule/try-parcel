# schema

with a nearly free-form text input, that just groups by prefix. order-sensitive

for example ethics:
Schema(
  index
  pars 1
    ...
  pars 2
    ...
  pars 3
    ...
  pars 4
    ...
  pars 5
    ...
)

fragment_comment (
fragment_id
user_id
span_start
span_end
language_code 2char
created_at
)

updating a schema -> deprecate the old one for the current book (just use created_at)
updating a fragment -> mark the old one as non-current (just use created_at)

(with an index fragment)
