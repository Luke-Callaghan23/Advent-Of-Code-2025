import os

mod_template = '''pub(crate) mod day_{};'''

fn_template = '''
pub fn star_one (input: String) -> String {
    todo!()
}

pub fn star_two (input: String) -> String {
    todo!()
}

'''

mods = []
for day in range(1, 26):
    for name in [ 'example.txt', 'full.txt' ]:
        path = os.path.join('data', str(day))
        os.makedirs(path, exist_ok=True)

        file = os.path.join(path, name)
        with open(file, 'w') as f:
            f.write('')

    fn_file = os.path.join('src', 'days', f'day_{day}.rs')
    with open(fn_file, 'w') as f:
        f.write(fn_template)

    mods.append(mod_template.format(day))

mod_file = os.path.join('src', 'days', 'mod.rs')
with open(mod_file, 'w') as f:
    f.write('\n'.join(mods))


