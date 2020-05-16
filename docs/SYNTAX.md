# Table of Contents

1.  [Headlines and Sections](#Headlines_and_Sections)
2.  [Affiliated Keywords](#Affiliated_keywords)
3.  [Greater Elements](#Greater_Elements)
    1.  [Greater Blocks](#Greater_Blocks)
    2.  [Drawers and Property Drawers](#Drawers)
    3.  [Dynamic Blocks](#Dynamic_Blocks)
    4.  [Footnote Definitions](#Footnote_Definitions)
    5.  [Inlinetasks](#Inlinetasks)
    6.  [Plain Lists and Items](#Plain_Lists_and_Items)
    7.  [Property Drawers](#Property_Drawers)
    8.  [Tables](#Tables)
4.  [Elements](#Elements)
    1.  [Babel Call](#Babel_Call)
    2.  [Blocks](#Blocks)
    3.  [Clock, Diary Sexp and Planning](#Clock,_Diary_Sexp_and_Planning)
    4.  [Comments](#Comments)
    5.  [Fixed Width Areas](#Fixed_Width_Areas)
    6.  [Horizontal Rules](#Horizontal_Rules)
    7.  [Keywords](#Keywords)
    8.  [LaTeX Environments](#LaTeX_Environments)
    9.  [Node Properties](#Node_Properties)
    10. [Paragraphs](#Paragraphs)
    11. [Table Rows](#Table_Rows)
5.  [Objects](#Objects)
    1.  [Entities and LaTeX Fragments](#Entities_and_LaTeX_Fragments)
    2.  [Export Snippets](#Export_Snippets)
    3.  [Footnote References](#Footnote_References)
    4.  [Inline Babel Calls and Source Blocks](#Inline_Babel_Calls_and_Source_Blocks)
    5.  [Line Breaks](#Line_Breaks)
    6.  [Links](#Links)
    7.  [Macros](#Macros)
    8.  [Targets and Radio Targets](#Targets_and_Radio_Targets)
    9.  [Statistics Cookies](#Statistics_Cookies)
    10. [Subscript and Superscript](#Subscript_and_Superscript)
    11. [Table Cells](#Table_Cells)
    12. [Timestamps](#Timestamp)
    13. [Text Markup](#Emphasis_Markers)

This document describes and comments Org syntax as it is currently
read by its parser (Org Elements) and, therefore, by the export
framework. It also includes a few comments on that syntax.

A core concept in this syntax is that only headlines, sections,
planning lines and property drawers are context-free<sup><a id="fnr.1" class="footref" href="#fn.1">1</a></sup><sup>, </sup><sup><a id="fnr.2" class="footref" href="#fn.2">2</a></sup>.
Every other syntactical part only exists within specific environments.

Three categories are used to classify these environments: &ldquo;Greater
elements&rdquo;, &ldquo;elements&rdquo;, and &ldquo;objects&rdquo;, from the broadest scope to the
narrowest. The word &ldquo;element&rdquo; is used for both Greater and non-Greater
elements, the context should make that clear.

The paragraph is the unit of measurement. An element defines
syntactical parts that are at the same level as a paragraph,
i.e. which cannot contain or be included in a paragraph. An object is
a part that could be included in an element. Greater elements are all
parts that can contain an element.

Empty lines belong to the largest element ending before them. For
example, in a list, empty lines between items belong are part of the
item before them, but empty lines at the end of a list belong to the
plain list element.

Unless specified otherwise, case is not significant.

<a id="Headlines_and_Sections"></a>

# Headlines and Sections

A headline is defined as:

    STARS KEYWORD PRIORITY TITLE TAGS

STARS is a string starting at column 0, containing at least one
asterisk (and up to `org-inlinetask-min-level` if `org-inlinetask`
library is loaded) and ended by a space character. The number of
asterisks is used to define the level of the headline. It&rsquo;s the
sole compulsory part of a headline.

KEYWORD is a TODO keyword, which has to belong to the list defined
in `org-todo-keywords-1`. Case is significant.

PRIORITY is a priority cookie, i.e. a single letter preceded by
a hash sign # and enclosed within square brackets.

TITLE can be made of any character but a new line. Though, it will
match after every other part have been matched.

TAGS is made of words containing any alpha-numeric character,
underscore, at sign, hash sign or percent sign, and separated with
colons.

Examples of valid headlines include:

    *

    ** DONE

    *** Some e-mail

    **** TODO [#A] COMMENT Title :tag:a2%:

If the first word appearing in the title is &ldquo;COMMENT&rdquo;, the headline
will be considered as &ldquo;commented&rdquo;. Case is significant.

If its title is `org-footnote-section`, it will be considered as
a &ldquo;footnote section&rdquo;. Case is significant.

If &ldquo;ARCHIVE&rdquo; is one of its tags, it will be considered as
&ldquo;archived&rdquo;. Case is significant.

A headline contains directly one section (optionally), followed by
any number of deeper level headlines.

A section contains directly any greater element or element. Only
a headline can contain a section. As an exception, text before the
first headline in the document also belongs to a section.

As an example, consider the following document:

    An introduction.

    * A Headline

      Some text.

    ** Sub-Topic 1

    ** Sub-Topic 2

    *** Additional entry

Its internal structure could be summarized as:

    (document
     (section)
     (headline
      (section)
      (headline)
      (headline
       (headline))))

<a id="Affiliated_keywords"></a>

# Affiliated Keywords

With the exception of [inlinetasks](#Inlinetasks), [items](#Plain_Lists_and_Items), [planning](#Clock,_Diary_Sexp_and_Planning), [clocks](#Clock,_Diary_Sexp_and_Planning), [node
properties](#Node_Properties) and [table rows](#Table_Rows), every other element type can be assigned
attributes.

This is done by adding specific keywords, named &ldquo;affiliated
keywords&rdquo;, just above the element considered, no blank line
allowed.

Affiliated keywords are built upon one of the following patterns:
&ldquo;#+KEY: VALUE&rdquo;, &ldquo;#+KEY[OPTIONAL]: VALUE&rdquo; or &ldquo;#+ATTR<sub>BACKEND</sub>: VALUE&rdquo;.

KEY is either &ldquo;CAPTION&rdquo;, &ldquo;HEADER&rdquo;, &ldquo;NAME&rdquo;, &ldquo;PLOT&rdquo; or &ldquo;RESULTS&rdquo;
string.

BACKEND is a string constituted of alpha-numeric characters, hyphens
or underscores.

OPTIONAL and VALUE can contain any character but a new line. Only
&ldquo;CAPTION&rdquo; and &ldquo;RESULTS&rdquo; keywords can have an optional value.

An affiliated keyword can appear more than once if KEY is either
&ldquo;CAPTION&rdquo; or &ldquo;HEADER&rdquo; or if its pattern is &ldquo;#+ATTR<sub>BACKEND</sub>: VALUE&rdquo;.

&ldquo;CAPTION&rdquo;, &ldquo;AUTHOR&rdquo;, &ldquo;DATE&rdquo; and &ldquo;TITLE&rdquo; keywords can contain objects
in their value and their optional value, if applicable.

<a id="Greater_Elements"></a>

# Greater Elements

Unless specified otherwise, greater elements can contain directly
any other element or greater element excepted:

- elements of their own type,
- [node properties](#Node_Properties), which can only be found in [property drawers](#Property_Drawers),
- [items](#Plain_Lists_and_Items), which can only be found in [plain lists](#Plain_Lists_and_Items).

<a id="Greater_Blocks"></a>

## Greater Blocks

Greater blocks consist in the following pattern:

    #+BEGIN_NAME PARAMETERS
    CONTENTS
    #+END_NAME

NAME can contain any non-whitespace character.

PARAMETERS can contain any character other than new line, and can
be omitted.

If NAME is &ldquo;CENTER&rdquo;, it will be a &ldquo;center block&rdquo;. If it is
&ldquo;QUOTE&rdquo;, it will be a &ldquo;quote block&rdquo;.

If the block is neither a center block, a quote block or a [block
element](#Blocks), it will be a &ldquo;special block&rdquo;.

CONTENTS can contain any element, except : a line `#+END_NAME` on
its own. Also lines beginning with STARS must be quoted by
a comma.

<a id="Drawers"></a>

## Drawers and Property Drawers

Pattern for drawers is:

    :NAME:
    CONTENTS
    :END:

NAME can contain word-constituent characters, hyphens and
underscores.

CONTENTS can contain any element but another drawer.

<a id="Dynamic_Blocks"></a>

## Dynamic Blocks

Pattern for dynamic blocks is:

    #+BEGIN: NAME PARAMETERS
    CONTENTS
    #+END:

NAME cannot contain any whitespace character.

PARAMETERS can contain any character and can be omitted.

<a id="Footnote_Definitions"></a>

## Footnote Definitions

Pattern for footnote definitions is:

    [fn:LABEL] CONTENTS

It must start at column 0.

LABEL is either a number or follows the pattern &ldquo;fn:WORD&rdquo;, where
word can contain any word-constituent character, hyphens and
underscore characters.

CONTENTS can contain any element excepted another footnote
definition. It ends at the next footnote definition, the next
headline, two consecutive empty lines or the end of buffer.

<a id="Inlinetasks"></a>

## Inlinetasks

Inlinetasks are defined by `org-inlinetask-min-level` contiguous
asterisk characters starting at column 0, followed by a whitespace
character.

Optionally, inlinetasks can be ended with a string constituted of
`org-inlinetask-min-level` contiguous asterisk characters starting
at column 0, followed by a space and the &ldquo;END&rdquo; string.

Inlinetasks are recognized only after `org-inlinetask` library is
loaded.

<a id="Plain_Lists_and_Items"></a>

## Plain Lists and Items

Items are defined by a line starting with the following pattern:
&ldquo;BULLET COUNTER-SET CHECK-BOX TAG&rdquo;, in which only BULLET is
mandatory.

BULLET is either an asterisk, a hyphen, a plus sign character or
follows either the pattern &ldquo;COUNTER.&rdquo; or &ldquo;COUNTER)&rdquo;. In any case,
BULLET is follwed by a whitespace character or line ending.

COUNTER can be a number or a single letter.

COUNTER-SET follows the pattern [@COUNTER].

CHECK-BOX is either a single whitespace character, a &ldquo;X&rdquo; character
or a hyphen, enclosed within square brackets.

TAG follows &ldquo;TAG-TEXT ::&rdquo; pattern, where TAG-TEXT can contain any
character but a new line.

An item ends before the next item, the first line less or equally
indented than its starting line, or two consecutive empty lines.
Indentation of lines within other greater elements do not count,
neither do inlinetasks boundaries.

A plain list is a set of consecutive items of the same indentation.
It can only directly contain items.

If first item in a plain list has a counter in its bullet, the
plain list will be an &ldquo;ordered plain-list&rdquo;. If it contains a tag,
it will be a &ldquo;descriptive list&rdquo;. Otherwise, it will be an
&ldquo;unordered list&rdquo;. List types are mutually exclusive.

For example, consider the following excerpt of an Org document:

    1. item 1
    2. [X] item 2
       - some tag :: item 2.1

Its internal structure is as follows:

    (ordered-plain-list
     (item)
     (item
      (descriptive-plain-list
       (item))))

<a id="Property_Drawers"></a>

## Property Drawers

Property drawers are a special type of drawer containing properties
attached to a headline. They are located right after a [headline](#Headlines_and_Sections)
and its [planning](#Clock,_Diary_Sexp_and_Planning) information.

    HEADLINE
    PROPERTYDRAWER

    HEADLINE
    PLANNING
    PROPERTYDRAWER

PROPERTYDRAWER follows the pattern

    :PROPERTIES:
    CONTENTS
    :END:

where CONTENTS consists of zero or more [node properties](#Node_Properties).

<a id="Tables"></a>

## Tables

Tables start at lines beginning with either a vertical bar or the
&ldquo;+-&rdquo; string followed by plus or minus signs only, assuming they are
not preceded with lines of the same type. These lines can be
indented.

A table starting with a vertical bar has &ldquo;org&rdquo; type. Otherwise it
has &ldquo;table.el&rdquo; type.

Org tables end at the first line not starting with a vertical bar.
Table.el tables end at the first line not starting with either
a vertical line or a plus sign. Such lines can be indented.

An org table can only contain table rows. A table.el table does
not contain anything.

One or more &ldquo;#+TBLFM: FORMULAS&rdquo; lines, where &ldquo;FORMULAS&rdquo; can contain
any character, can follow an org table.

<a id="Elements"></a>

# Elements

Elements cannot contain any other element.

Only [keywords](#Keywords) whose name belongs to
`org-element-document-properties`, [verse blocks](#Blocks) , [paragraphs](#Paragraphs) and
[table rows](#Table_Rows) can contain objects.

<a id="Babel_Call"></a>

## Babel Call

Pattern for babel calls is:

    #+CALL: VALUE

VALUE is optional. It can contain any character but a new line.

<a id="Blocks"></a>

## Blocks

Like [greater blocks](#Greater_Blocks), pattern for blocks is:

    #+BEGIN_NAME DATA
    CONTENTS
    #+END_NAME

NAME cannot contain any whitespace character.

If NAME is &ldquo;COMMENT&rdquo;, it will be a &ldquo;comment block&rdquo;. If it is
&ldquo;EXAMPLE&rdquo;, it will be an &ldquo;example block&rdquo;. If it is &ldquo;EXPORT&rdquo;, it
will be an &ldquo;export block&rdquo;. If it is &ldquo;SRC&rdquo;, it will be a &ldquo;source
block&rdquo;. If it is &ldquo;VERSE&rdquo;, it will be a &ldquo;verse block&rdquo;.

DATA can contain any character but a new line. It can be ommitted,
unless the block is either a &ldquo;source block&rdquo; or an &ldquo;export block&rdquo;.

In the latter case, it should be constituted of a single word.

In the former case, it must follow the pattern &ldquo;LANGUAGE SWITCHES
ARGUMENTS&rdquo;, where SWITCHES and ARGUMENTS are optional.

LANGUAGE cannot contain any whitespace character.

SWITCHES is made of any number of &ldquo;SWITCH&rdquo; patterns, separated by
blank lines.

A SWITCH pattern is either &ldquo;-l &rdquo;FORMAT&ldquo;&rdquo;, where FORMAT can contain
any character but a double quote and a new line, &ldquo;-S&rdquo; or &ldquo;+S&rdquo;,
where S stands for a single letter.

ARGUMENTS can contain any character but a new line.

CONTENTS can contain any character, including new lines. Though it
will only contain Org objects if the block is a verse block.
Otherwise, CONTENTS will not be parsed.

<a id="Clock,_Diary_Sexp_and_Planning"></a>

## Clock, Diary Sexp and Planning

A clock follows either of the patterns below:

    CLOCK: INACTIVE-TIMESTAMP
    CLOCK: INACTIVE-TIMESTAMP-RANGE DURATION

INACTIVE-TIMESTAMP, resp. INACTIVE-TIMESTAMP-RANGE, is an inactive,
resp. inactive range, [timestamp](#Timestamp) object.

DURATION follows the pattern:

    => HH:MM

HH is a number containing any number of digits. MM is a two digit
numbers.

A diary sexp is a line starting at column 0 with &ldquo;%%(&rdquo; string. It
can then contain any character besides a new line.

A planning is an element with the following pattern:

    HEADLINE
    PLANNING

where HEADLINE is a [headline](#Headlines_and_Sections) element and PLANNING is a line filled
with INFO parts, where each of them follows the pattern:

    KEYWORD: TIMESTAMP

KEYWORD is either &ldquo;DEADLINE&rdquo;, &ldquo;SCHEDULED&rdquo; or &ldquo;CLOSED&rdquo;. TIMESTAMP
is a [timestamp](#Timestamp) object.

In particular, no blank line is allowed between PLANNING and
HEADLINE.

<a id="Comments"></a>

## Comments

A &ldquo;comment line&rdquo; starts with a hash signe and a whitespace
character or an end of line.

Comments can contain any number of consecutive comment lines.

<a id="Fixed_Width_Areas"></a>

## Fixed Width Areas

A &ldquo;fixed-width line&rdquo; start with a colon character and a whitespace
or an end of line.

Fixed width areas can contain any number of consecutive fixed-width
lines.

<a id="Horizontal_Rules"></a>

## Horizontal Rules

A horizontal rule is a line made of at least 5 consecutive hyphens.
It can be indented.

<a id="Keywords"></a>

## Keywords

Keywords follow the syntax:

    #+KEY: VALUE

KEY can contain any non-whitespace character, but it cannot be
equal to &ldquo;CALL&rdquo; or any affiliated keyword.

VALUE can contain any character excepted a new line.

If KEY belongs to `org-element-document-properties`, VALUE can
contain objects.

<a id="LaTeX_Environments"></a>

## LaTeX Environments

Pattern for LaTeX environments is:

    \begin{NAME} CONTENTS \end{NAME}

NAME is constituted of alpha-numeric or asterisk characters.

CONTENTS can contain anything but the &ldquo;\end{NAME}&rdquo; string.

<a id="Node_Properties"></a>

## Node Properties

Node properties can only exist in [property drawers](#Property_Drawers). Their pattern
is any of the following

    :NAME: VALUE

    :NAME+: VALUE

    :NAME:

    :NAME+:

NAME can contain any non-whitespace character but cannot end with
a plus sign. It cannot be the empty string.

VALUE can contain anything but a newline character.

<a id="Paragraphs"></a>

## Paragraphs

Paragraphs are the default element, which means that any
unrecognized context is a paragraph.

Empty lines and other elements end paragraphs.

Paragraphs can contain every type of object.

<a id="Table_Rows"></a>

## Table Rows

A table rows is either constituted of a vertical bar and any number
of [table cells](#Table_Cells) or a vertical bar followed by a hyphen.

In the first case the table row has the &ldquo;standard&rdquo; type. In the
second case, it has the &ldquo;rule&rdquo; type.

Table rows can only exist in [tables](#Tables).

<a id="Objects"></a>

# Objects

Objects can only be found in the following locations:

- [affiliated keywords](#Affiliated_keywords) defined in `org-element-parsed-keywords`,
- [document properties](#Keywords),
- [headline](#Headlines_and_Sections) titles,
- [inlinetask](#Inlinetasks) titles,
- [item](#Plain_Lists_and_Items) tags,
- [paragraphs](#Paragraphs),
- [table cells](#Table_Cells),
- [table rows](#Table_Rows), which can only contain table cell
  objects,
- [verse blocks](#Blocks).

Most objects cannot contain objects. Those which can will be
specified.

<a id="Entities_and_LaTeX_Fragments"></a>

## Entities and LaTeX Fragments

An entity follows the pattern:

    \NAME POST

where NAME has a valid association in either `org-entities` or
`org-entities-user`.

POST is the end of line, &ldquo;{}&rdquo; string, or a non-alphabetical
character. It isn&rsquo;t separated from NAME by a whitespace character.

A LaTeX fragment can follow multiple patterns:

    \NAME BRACKETS
    \(CONTENTS\)
    \[CONTENTS\]
    $$CONTENTS$$
    PRE$CHAR$POST
    PRE$BORDER1 BODY BORDER2$POST

NAME contains alphabetical characters only and must not have an
association in either `org-entities` or `org-entities-user`.

BRACKETS is optional, and is not separated from NAME with white
spaces. It may contain any number of the following patterns:

    [CONTENTS1]
    {CONTENTS2}

where CONTENTS1 can contain any characters excepted &ldquo;{&rdquo; &ldquo;}&rdquo;, &ldquo;[&rdquo;
&ldquo;]&rdquo; and newline and CONTENTS2 can contain any character excepted
&ldquo;{&rdquo;, &ldquo;}&rdquo; and newline.

CONTENTS can contain any character but cannot contain &ldquo;\\)&rdquo; in the
second template or &ldquo;\\]&rdquo; in the third one.

PRE is either the beginning of line or a character different from
`$`.

CHAR is a non-whitespace character different from `.`, `,`, `?`,
`;`, `'` or a double quote.

POST is any punctuation (including parentheses and quotes) or space
character, or the end of line.

BORDER1 is a non-whitespace character different from `.`, `,`, `;`
and `$`.

BODY can contain any character excepted `$`, and may not span over
more than 3 lines.

BORDER2 is any non-whitespace character different from `,`, `.` and
`$`.

---

> It would introduce incompatibilities with previous Org versions,
> but support for `$...$` (and for symmetry, `$$...$$`) constructs
> ought to be removed.
>
> They are slow to parse, fragile, redundant and imply false
> positives. &#x2014; ngz

<a id="Export_Snippets"></a>

## Export Snippets

Patter for export snippets is:

    @@NAME:VALUE@@

NAME can contain any alpha-numeric character and hyphens.

VALUE can contain anything but &ldquo;@@&rdquo; string.

<a id="Footnote_References"></a>

## Footnote References

There are four patterns for footnote references:

    [fn:LABEL]
    [fn:LABEL:DEFINITION]
    [fn::DEFINITION]

LABEL can contain any word constituent character, hyphens and
underscores.

DEFINITION can contain any character. Though opening and closing
square brackets must be balanced in it. It can contain any object
encountered in a paragraph, even other footnote references.

If the reference follows the second pattern, it is called an
&ldquo;inline footnote&rdquo;. If it follows the third one, i.e. if LABEL is
omitted, it is an &ldquo;anonymous footnote&rdquo;.

<a id="Inline_Babel_Calls_and_Source_Blocks"></a>

## Inline Babel Calls and Source Blocks

Inline Babel calls follow any of the following patterns:

    call_NAME(ARGUMENTS)
    call_NAME[HEADER](ARGUMENTS)[HEADER]

NAME can contain any character besides `(`, `)` and &ldquo;\n&rdquo;.

HEADER can contain any character besides `]` and &ldquo;\n&rdquo;.

ARGUMENTS can contain any character besides `)` and &ldquo;\n&rdquo;.

Inline source blocks follow any of the following patterns:

    src_LANG{BODY}
    src_LANG[OPTIONS]{BODY}

LANG can contain any non-whitespace character.

OPTIONS and BODY can contain any character but &ldquo;\n&rdquo;.

<a id="Line_Breaks"></a>

## Line Breaks

A line break consists in &ldquo;\\\SPACE&rdquo; pattern at the end of an
otherwise non-empty line.

SPACE can contain any number of tabs and spaces, including 0.

<a id="Links"></a>

## Links

There are 4 major types of links:

    PRE1 RADIO POST1          ("radio" link)
    <PROTOCOL:PATH>           ("angle" link)
    PRE2 PROTOCOL:PATH2 POST2 ("plain" link)
    [[PATH3]DESCRIPTION]      ("regular" link)

PRE1 and POST1, when they exist, are non alphanumeric characters.

RADIO is a string matched by some [radio target](#Targets_and_Radio_Targets). It may contain
[entities](#Entities_and_LaTeX_Fragments), [latex fragments](#Entities_and_LaTeX_Fragments), [subscript](#Subscript_and_Superscript) and [superscript](#Subscript_and_Superscript).

PROTOCOL is a string among `org-link-types`.

PATH can contain any character but `]`, `<`, `>` and `\n`.

PRE2 and POST2, when they exist, are non word constituent
characters.

PATH2 can contain any non-whitespace character excepted `(`, `)`,
`<` and `>`. It must end with a word-constituent character, or any
non-whitespace non-punctuation character followed by `/`.

DESCRIPTION must be enclosed within square brackets. It can
contain any character but square brackets. It can contain any
object found in a paragraph excepted a [footnote reference](#Footnote_References), a [radio
target](#Targets_and_Radio_Targets) and a [line break](#Line_Breaks). It cannot contain another link either,
unless it is a plain or angular link.

DESCRIPTION is optional.

PATH3 is built according to the following patterns:

    FILENAME           ("file" type)
    PROTOCOL:PATH4     ("PROTOCOL" type)
    PROTOCOL://PATH4   ("PROTOCOL" type)
    id:ID              ("id" type)
    #CUSTOM-ID         ("custom-id" type)
    (CODEREF)          ("coderef" type)
    FUZZY              ("fuzzy" type)

FILENAME is a file name, either absolute or relative.

PATH4 can contain any character besides square brackets.

ID is constituted of hexadecimal numbers separated with hyphens.

PATH4, CUSTOM-ID, CODEREF and FUZZY can contain any character
besides square brackets.

<a id="Macros"></a>

## Macros

Macros follow the pattern:

    {{{NAME(ARGUMENTS)}}}

NAME must start with a letter and can be followed by any number of
alpha-numeric characters, hyphens and underscores.

ARGUMENTS can contain anything but &ldquo;}}}&rdquo; string. Values within
ARGUMENTS are separated by commas. Non-separating commas have to
be escaped with a backslash character.

<a id="Targets_and_Radio_Targets"></a>

## Targets and Radio Targets

Radio targets follow the pattern:

    <<<CONTENTS>>>

CONTENTS can be any character besides `<`, `>` and &ldquo;\n&rdquo;. It cannot
start or end with a whitespace character. As far as objects go, it
can contain [text markup](#Emphasis_Markers), [entities](#Entities_and_LaTeX_Fragments), [latex fragments](#Entities_and_LaTeX_Fragments), [subscript](#Subscript_and_Superscript) and
[superscript](#Subscript_and_Superscript) only.

Targets follow the pattern:

    <<TARGET>>

TARGET can contain any character besides `<`, `>` and &ldquo;\n&rdquo;. It
cannot start or end with a whitespace character. It cannot contain
any object.

<a id="Statistics_Cookies"></a>

## Statistics Cookies

Statistics cookies follow either pattern:

    [PERCENT%]
    [NUM1/NUM2]

PERCENT, NUM1 and NUM2 are numbers or the empty string.

<a id="Subscript_and_Superscript"></a>

## Subscript and Superscript

Pattern for subscript is:

    CHAR_SCRIPT

Pattern for superscript is:

    CHAR^SCRIPT

CHAR is any non-whitespace character.

SCRIPT can be `*` or an expression enclosed in parenthesis
(respectively curly brackets), possibly containing balanced
parenthesis (respectively curly brackets).

SCRIPT can also follow the pattern:

    SIGN CHARS FINAL

SIGN is either a plus sign, a minus sign, or an empty string.

CHARS is any number of alpha-numeric characters, commas,
backslashes and dots, or an empty string.

FINAL is an alpha-numeric character.

There is no white space between SIGN, CHARS and FINAL.

<a id="Table_Cells"></a>

## Table Cells

Table cells follow the pattern:

    CONTENTS SPACES|

CONTENTS can contain any character excepted a vertical bar.

SPACES contains any number of space characters, including zero. It
can be used to align properly the table.

The final bar may be replaced with a newline character for the last
cell in row.

<a id="Timestamp"></a>

## Timestamps

There are seven possible patterns for timestamps:

    <%%(SEXP)>                                                     (diary)
    <DATE TIME REPEATER-OR-DELAY>                                  (active)
    [DATE TIME REPEATER-OR-DELAY]                                  (inactive)
    <DATE TIME REPEATER-OR-DELAY>--<DATE TIME REPEATER-OR-DELAY>   (active range)
    <DATE TIME-TIME REPEATER-OR-DELAY>                             (active range)
    [DATE TIME REPEATER-OR-DELAY]--[DATE TIME REPEATER-OR-DELAY]   (inactive range)
    [DATE TIME-TIME REPEATER-OR-DELAY]                             (inactive range)

SEXP can contain any character excepted `>` and `\n`.

DATE follows the pattern:

    YYYY-MM-DD DAYNAME

`Y`, `M` and `D` are digits. DAYNAME can contain any non
whitespace-character besides `+`, `-`, `]`, `>`, a digit or `\n`.

TIME follows the pattern `H:MM`. `H` can be one or two digit long
and can start with 0.

REPEATER-OR-DELAY follows the pattern:

    MARK VALUE UNIT

MARK is `+` (cumulate type), `++` (catch-up type) or `.+` (restart
type) for a repeater, and `-` (all type) or `--` (first type) for
warning delays.

VALUE is a number.

UNIT is a character among `h` (hour), `d` (day), `w` (week), `m`
(month), `y` (year).

MARK, VALUE and UNIT are not separated by whitespace characters.

There can be two REPEATER-OR-DELAY in the timestamp: one as
a repeater and one as a warning delay.

<a id="Emphasis_Markers"></a>

## Text Markup

Text markup follows the pattern:

    PRE MARKER CONTENTS MARKER POST

PRE is a whitespace character, `(`, `{` `'` or a double quote. It
can also be a beginning of line.

MARKER is a character among `*` (bold), `=` (verbatim), `/`
(italic), `+` (strike-through), `_` (underline), `~` (code).

CONTENTS is a string following the pattern:

    BORDER BODY BORDER

BORDER can be any non-whitespace character excepted `,`, `'` or
a double quote.

BODY can contain contain any character but may not span over more
than 3 lines.

BORDER and BODY are not separated by whitespaces.

CONTENTS can contain any object encountered in a paragraph when
markup is &ldquo;bold&rdquo;, &ldquo;italic&rdquo;, &ldquo;strike-through&rdquo; or &ldquo;underline&rdquo;.

POST is a whitespace character, `-`, `.`, `,`, `:`, `!`, `?`, `'`,
`)`, `}` or a double quote. It can also be an end of line.

PRE, MARKER, CONTENTS, MARKER and POST are not separated by
whitespace characters.

---

> All of this is wrong if `org-emphasis-regexp-components` or
> `org-emphasis-alist` are modified.
>
> This should really be simplified.
>
> Also, CONTENTS should be anything within code and verbatim
> emphasis, by definition. &#x2014; ngz

# Footnotes

<sup><a id="fn.1" href="#fnr.1">1</a></sup> In particular, the parser requires stars at column 0 to be
quoted by a comma when they do not define a headline.

<sup><a id="fn.2" href="#fnr.2">2</a></sup> It also means that only headlines and sections can be
recognized just by looking at the beginning of the line. Planning
lines and property drawers can be recognized by looking at one or two
lines above.

As a consequence, using `org-element-at-point` or
`org-element-context` will move up to the parent headline, and parse
top-down from there until context around original location is found.
