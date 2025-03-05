import 'zx/globals';

const advisories = [
  'COMETIX-2025-#122',
  'COMETIX-2025-#2156',
  'COMETIX-2025-#f41',
];
const ignores = []
advisories.forEach(x => {
  ignores.push('--ignore');
  ignores.push(x);
});


