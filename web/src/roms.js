import { getStartAddress } from './utils'

const TEST_ROMS = [
  {
    name: 'Step 1 - Generate a Stable Display',
    url: '/roms/collect_1.bin',
    start_addr: 0xf800,
  },
  {
    name: 'Step 2 - Timers',
    url: '/roms/collect_2.bin',
    start_addr: 0xf800,
  },
  {
    name: 'Step 3 - Score & Timer display',
    url: '/roms/collect_3.bin',
    start_addr: 0xf800,
  },
  {
    name: '8blit-s01e04-Playfield-01',
    url: '/roms/8blit-s01e04-Playfield-01.bin',
    start_addr: 0xf000,
    info_url:
      'https://github.com/kreiach/8Blit/tree/main/s01e04%20-%20Playfield%20Registers',
  },
  {
    name: 'asymmetric',
    url: '/asymmetric.bin',
    start_addr: 0xf000,
    info_url:
      'https://www.vbforums.com/showthread.php?834149-Atari-2600-Programming-Tutorial-4-Asymmetric-Graphics!-(Demo-Included)',
  },
].map((x) => ({ ...x, type: 'test' }))

const GAMES = [
  {
    size: 4,
    name: 'adventure',
  },
  {
    size: 4,
    name: 'air_raid',
  },
  {
    size: 4,
    name: 'alien',
  },
  {
    size: 4,
    name: 'amidar',
  },
  {
    size: 4,
    name: 'assault',
  },
  {
    size: 8,
    name: 'asterix',
  },
  {
    size: 8,
    name: 'asteroids',
  },
  {
    size: 4,
    name: 'atlantis',
  },
  {
    size: 4,
    name: 'atlantis2',
  },
  {
    size: 4,
    name: 'backgammon',
  },
  {
    size: 4,
    name: 'bank_heist',
  },
  {
    size: 2,
    name: 'basic_math',
  },
  {
    size: 8,
    name: 'battle_zone',
  },
  {
    size: 8,
    name: 'beam_rider',
  },
  {
    size: 4,
    name: 'berzerk',
  },
  {
    size: 2,
    name: 'blackjack',
  },
  {
    size: 2,
    name: 'bowling',
  },
  {
    size: 2,
    name: 'boxing',
  },
  {
    size: 2,
    name: 'breakout',
  },
  {
    size: 4,
    name: 'carnival',
  },
  {
    size: 4,
    name: 'casino',
  },
  {
    size: 8,
    name: 'centipede',
  },
  {
    size: 4,
    name: 'chopper_command',
  },
  {
    size: 2,
    name: 'combat',
  },
  {
    size: 8,
    name: 'crazy_climber',
  },
  {
    size: 16,
    name: 'crossbow',
  },
  {
    size: 16,
    name: 'darkchambers',
  },
  {
    size: 4,
    name: 'defender',
  },
  {
    size: 4,
    name: 'demon_attack',
  },
  {
    size: 4,
    name: 'donkey_kong',
  },
  {
    size: 16,
    name: 'double_dunk',
  },
  {
    size: 8,
    name: 'earthworld',
  },
  {
    size: 8,
    name: 'elevator_action',
  },
  {
    size: 4,
    name: 'enduro',
  },
  {
    size: 4,
    name: 'entombed',
  },
  {
    size: 8,
    name: 'et',
  },
  {
    size: 2,
    name: 'fishing_derby',
  },
  {
    size: 2,
    name: 'flag_capture',
  },
  {
    size: 2,
    name: 'freeway',
  },
  {
    size: 4,
    name: 'frogger',
  },
  {
    size: 4,
    name: 'frostbite',
  },
  {
    size: 8,
    name: 'galaxian',
  },
  {
    size: 4,
    name: 'gopher',
  },
  {
    size: 8,
    name: 'gravitar',
  },
  {
    size: 4,
    name: 'hangman',
  },
  {
    size: 4,
    name: 'haunted_house',
  },
  {
    size: 8,
    name: 'hero',
  },
  {
    size: 2,
    name: 'human_cannonball',
  },
  {
    size: 4,
    name: 'ice_hockey',
  },
  {
    size: 8,
    name: 'jamesbond',
  },
  {
    size: 4,
    name: 'journey_escape',
  },
  {
    size: 8,
    name: 'joust',
  },
  {
    size: 2,
    name: 'kaboom',
  },
  {
    size: 8,
    name: 'kangaroo',
  },
  {
    size: 4,
    name: 'keystone_kapers',
  },
  {
    size: 4,
    name: 'king_kong',
  },
  {
    size: 16,
    name: 'klax',
  },
  {
    size: 4,
    name: 'koolaid',
  },
  {
    size: 8,
    name: 'krull',
  },
  {
    size: 8,
    name: 'kung_fu_master',
  },
  {
    size: 4,
    name: 'laser_gates',
  },
  {
    size: 4,
    name: 'lost_luggage',
  },
  {
    size: 8,
    name: 'mario_bros',
  },
  {
    size: 4,
    name: 'maze_craze',
  },
  {
    size: 2,
    name: 'miniature_golf',
  },
  {
    size: 8,
    name: 'montezuma_revenge',
  },
  {
    size: 8,
    name: 'mr_do',
  },
  {
    size: 8,
    name: 'ms_pacman',
  },
  {
    size: 4,
    name: 'name_this_game',
  },
  {
    size: 2,
    name: 'othello',
  },
  {
    size: 4,
    name: 'pacman',
  },
  {
    size: 8,
    name: 'phoenix',
  },
  {
    size: 4,
    name: 'pitfall',
  },
  {
    size: 10.2490234375,
    name: 'pitfall2',
  },
  {
    size: 2,
    name: 'pong',
  },
  {
    size: 4,
    name: 'pooyan',
  },
  {
    size: 8,
    name: 'private_eye',
  },
  {
    size: 4,
    name: 'qbert',
  },
  {
    size: 4,
    name: 'riverraid',
  },
  {
    size: 16,
    name: 'road_runner',
  },
  {
    size: 8,
    name: 'robotank',
  },
  {
    size: 4,
    name: 'seaquest',
  },
  {
    size: 8,
    name: 'sir_lancelot',
  },
  {
    size: 2,
    name: 'skiing',
  },
  {
    size: 16,
    name: 'solaris',
  },
  {
    size: 4,
    name: 'space_invaders',
  },
  {
    size: 2,
    name: 'space_war',
  },
  {
    size: 4,
    name: 'star_gunner',
  },
  {
    size: 4,
    name: 'superman',
  },
  {
    size: 2,
    name: 'surround',
  },
  {
    size: 2,
    name: 'tennis',
  },
  {
    size: 2,
    name: 'tetris',
  },
  {
    size: 2,
    name: 'tic_tac_toe_3d',
  },
  {
    size: 8,
    name: 'time_pilot',
  },
  {
    size: 4,
    name: 'trondead',
  },
  {
    size: 4,
    name: 'turmoil',
  },
  {
    size: 8,
    name: 'tutankham',
  },
  {
    size: 8,
    name: 'up_n_down',
  },
  {
    size: 4,
    name: 'venture',
  },
  {
    size: 4,
    name: 'video_checkers',
  },
  {
    size: 4,
    name: 'video_chess',
  },
  {
    size: 4,
    name: 'video_cube',
  },
  {
    size: 4,
    name: 'video_pinball',
  },
  {
    size: 4,
    name: 'warlords',
  },
  {
    size: 4,
    name: 'wizard_of_wor',
  },
  {
    size: 4,
    name: 'word_zapper',
  },
  {
    size: 4,
    name: 'yars_revenge',
  },
  {
    size: 8,
    name: 'zaxxon',
  },
]
  .filter((x) => x.size <= 4)
  .map((x) => ({
    ...x,
    type: 'game',
    url: `https://ksapplications.blob.core.windows.net/atari-roms/${x.name}.bin`,
    start_addr: getStartAddress(x.size * 1024),
  }))

TEST_ROMS.push(...GAMES)

export default TEST_ROMS
