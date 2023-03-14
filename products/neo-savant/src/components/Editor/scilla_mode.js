/* eslint-disable */
ace.define(
  'ace/mode/scilla_highlight_rules',
  ['require', 'exports', 'module', 'ace/lib/oop', 'ace/mode/text_highlight_rules'],
  function(acequire, exports, module) {
    'use strict';

    var oop = acequire('../lib/oop');
    var TextHighlightRules = acequire('./text_highlight_rules').TextHighlightRules;

    var ScillaHighlightRules = function() {
      var keywords =
        'contract|library|import|builtin|' +
        'transition|fun|field|tfun|in|end|' +
        'match|with|event|send|accept';

      var storage = 'let|fun|tfun|';

      var builtinCtor = 'Map|Emp|Cons|Nil|Some|None|False|True';

      var builtinConstants =
        'Bool|Option|List|' +
        'BNum|BLOCKNUMBER|Message|Event|' +
        'Uint256|Uint128|Uint64|Uint32|' +
        'Int256|Int128|Int64|Int32|' +
        'ByStr20|ByStr32|ByStr33';

      var builtinFunctions =
        'eq|concat|substr|' +
        'blt|badd|' +
        'dist|sha256hash|to_bystr|schnorr_gen_key_pair|schnorr_sign|schnorr_verify|' +
        'contains|put|get|remove|to_list|' +
        'lt|add|sub|mul|div|rem|to_int32|to_int64|to_int128|to_int256|to_nat|';

      var keywordMapper = this.createKeywordMapper(
        {
          keyword: keywords,
          'constant.language': builtinConstants,
          'storage.type': storage,
          'support.function': builtinFunctions,
          'variable.language': builtinCtor,
        },
        'identifier',
      );

      var decimalInteger = '(?:(?:[1-9]\\d*)|(?:0))';
      var hexInteger = '(?:0[xX][\\dA-Fa-f]+)';
      var integer = '(?:' + decimalInteger + '|' + hexInteger + ')';

      var intPart = '(?:\\d+)';

      this.$rules = {
        start: [
          {
            token: 'comment',
            regex: '\\(\\*.*?\\*\\)\\s*?$',
          },
          {
            token: 'comment',
            regex: '\\(\\*.*',
            next: 'comment',
          },
          {
            token: 'string', // single line
            regex: '["](?:(?:\\\\.)|(?:[^"\\\\]))*?["]',
          },
          {
            token: 'string', // single char
            regex: "'.'",
          },
          {
            token: 'string', // " string
            regex: '"',
            next: 'qstring',
          },
          {
            token: 'constant.numeric', // integer
            regex: integer + '\\b',
          },
          {
            token: keywordMapper,
            regex: '[a-zA-Z_$][a-zA-Z0-9_$]*\\b',
          },
          {
            token: 'keyword.operator',
            regex:
              '\\+\\.|\\-\\.|\\*\\.|\\/\\.|#|;|\\+|\\-|\\*|\\*\\*\\/|\\/\\/|%|<<|>>|&|\\||\\^|~|<|>|<=|=>|==|!=|<>|<-|=|:=',
          },
          {
            token: 'paren.lparen',
            regex: '[[({]',
          },
          {
            token: 'paren.rparen',
            regex: '[\\])}]',
          },
          {
            token: 'text',
            regex: '\\s+',
          },
        ],
        comment: [
          {
            token: 'comment', // closing comment
            regex: '\\*\\)',
            next: 'start',
          },
          {
            defaultToken: 'comment',
          },
        ],
        qstring: [
          {
            token: 'string',
            regex: '"',
            next: 'start',
          },
          {
            token: 'string',
            regex: '.+',
          },
        ],
      };
    };

    oop.inherits(ScillaHighlightRules, TextHighlightRules);

    exports.ScillaHighlightRules = ScillaHighlightRules;
  },
);

ace.define(
  'ace/mode/matching_brace_outdent',
  ['require', 'exports', 'module', 'ace/range'],
  function(acequire, exports, module) {
    'use strict';

    var Range = acequire('../range').Range;

    var MatchingBraceOutdent = function() {};

    (function() {
      this.checkOutdent = function(line, input) {
        if (!/^\s+$/.test(line)) return false;

        return /^\s*\}/.test(input);
      };

      this.autoOutdent = function(doc, row) {
        var line = doc.getLine(row);
        var match = line.match(/^(\s*\})/);

        if (!match) return 0;

        var column = match[1].length;
        var openBracePos = doc.findMatchingBracket({ row: row, column: column });

        if (!openBracePos || openBracePos.row == row) return 0;

        var indent = this.$getIndent(doc.getLine(openBracePos.row));
        doc.replace(new Range(row, 0, row, column - 1), indent);
      };

      this.$getIndent = function(line) {
        return line.match(/^\s*/)[0];
      };
    }.call(MatchingBraceOutdent.prototype));

    exports.MatchingBraceOutdent = MatchingBraceOutdent;
  },
);

ace.define(
  'ace/mode/scilla',
  [
    'require',
    'exports',
    'module',
    'ace/lib/oop',
    'ace/mode/text',
    'ace/mode/scilla_highlight_rules',
    'ace/mode/matching_brace_outdent',
    'ace/range',
  ],
  function(acequire, exports, module) {
    'use strict';

    var oop = acequire('../lib/oop');
    var TextMode = acequire('./text').Mode;
    var ScillaHighlightRules = acequire('./scilla_highlight_rules').ScillaHighlightRules;
    var MatchingBraceOutdent = acequire('./matching_brace_outdent').MatchingBraceOutdent;
    var Range = acequire('../range').Range;

    var Mode = function() {
      this.HighlightRules = ScillaHighlightRules;
      this.$behaviour = this.$defaultBehaviour;

      this.$outdent = new MatchingBraceOutdent();
    };
    oop.inherits(Mode, TextMode);

    var indenter = /(?:[({[=:]|[-=]>|\b(?:else|try|with))\s*$/;

    (function() {
      this.toggleCommentLines = function(state, doc, startRow, endRow) {
        var i, line;
        var outdent = true;
        var re = /^\s*\(\*(.*)\*\)/;

        for (i = startRow; i <= endRow; i++) {
          if (!re.test(doc.getLine(i))) {
            outdent = false;
            break;
          }
        }

        var range = new Range(0, 0, 0, 0);
        for (i = startRow; i <= endRow; i++) {
          line = doc.getLine(i);
          range.start.row = i;
          range.end.row = i;
          range.end.column = line.length;

          doc.replace(range, outdent ? line.match(re)[1] : '(*' + line + '*)');
        }
      };

      this.getNextLineIndent = function(state, line, tab) {
        var indent = this.$getIndent(line);
        var tokens = this.getTokenizer().getLineTokens(line, state).tokens;

        if (
          !(tokens.length && tokens[tokens.length - 1].type === 'comment') &&
          state === 'start' &&
          indenter.test(line)
        )
          indent += tab;
        return indent;
      };

      this.checkOutdent = function(state, line, input) {
        return this.$outdent.checkOutdent(line, input);
      };

      this.autoOutdent = function(state, doc, row) {
        this.$outdent.autoOutdent(doc, row);
      };

      this.$id = 'ace/mode/scilla';
    }.call(Mode.prototype));

    exports.Mode = Mode;
  },
);
