import { getStartAddress } from './utils'

const TEST_ROMS = [
  {
    name: 'collect-01-StableDisplay',
    url: '/roms/collect-01-StableDisplay.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-02-Timer',
    url: '/roms/collect-02-Timer.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-03-ScoreAndTimerDisplay',
    url: '/roms/collect-03-ScoreAndTimerDisplay.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-04-2LineKernel',
    url: '/roms/collect-04-2LineKernel.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-05-AutomateVerticalDelay',
    url: '/roms/collect-05-AutomateVerticalDelay.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-06-SpecChange',
    url: '/roms/collect-06-SpecChange.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-07-DrawThePlayfield',
    url: '/roms/collect-07-DrawThePlayfield.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-08-SelectAndResetSupport',
    url: '/roms/collect-08-SelectAndResetSupport.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-09-GameVariations',
    url: '/roms/collect-09-GameVariations.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-10-RandomNumbers',
    url: '/roms/collect-10-RandomNumbers.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-11-AddTheBallObject',
    url: '/roms/collect-11-AddTheBallObject.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-12-AddTheMissileObjects',
    url: '/roms/collect-12-AddTheMissileObjects.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-13-AddSoundEffects',
    url: '/roms/collect-13-AddSoundEffects.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-14-AddAnimation',
    url: '/roms/collect-14-AddAnimation.bin',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: 'collect-15-CollectMini',
    url: '/roms/collect-15-CollectMini.bin  ',
    start_addr: 0xf800,
    info_url: '',
  },
  {
    name: '8blit-s01e02-background',
    url: '/roms/8blit-s01e02-background.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e03-background',
    url: '/roms/8blit-s01e03-background.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: 'asymmetric-playfield',
    url: '/roms/asymmetric.bin',
    start_addr: 0xf000,
    info_url:
      'https://www.vbforums.com/showthread.php?834149-Atari-2600-Programming-Tutorial-4-Asymmetric-Graphics!-(Demo-Included)',
  },
  {
    name: '8blit-s01e04-Playfield-01',
    url: '/roms/8blit-s01e04-Playfield-01.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e05-Ex0-Playfield-Box',
    url: '/roms/8blit-s01e05-Ex0-Playfield-Box.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e05-Ex1-Playfield-Box',
    url: '/roms/8blit-s01e05-Ex1-Playfield-Box.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e05-Ex2-Playfield-Box',
    url: '/roms/8blit-s01e05-Ex2-Playfield-Box.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e05-Ex3-Playfield-Box',
    url: '/roms/8blit-s01e05-Ex3-Playfield-Box.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e06-Ex1-First-Sprite',
    url: '/roms/8blit-s01e06-Ex1-First-Sprite.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e06-Ex2-Course Movement',
    url: '/roms/8blit-s01e06-Ex2-Course Movement.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e06-Ex3-Fine Movement',
    url: '/roms/8blit-s01e06-Ex3-Fine Movement.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e06-Ex4-Two Dimensional Sprite',
    url: '/roms/8blit-s01e06-Ex4-Two Dimensional Sprite.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex0-Cat (inefficient)',
    url: '/roms/8blit-s01e07-Ex0-Cat (inefficient).bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex1-Cat',
    url: '/roms/8blit-s01e07-Ex1-Cat.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex2-Cat and Dog',
    url: '/roms/8blit-s01e07-Ex2-Cat and Dog.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex3-Cat and Dog (unstable) ',
    url: '/roms/8blit-s01e07-Ex3-Cat and Dog (unstable) .bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex4-Cat and Dog Vertical Delay (unstable)',
    url: '/roms/8blit-s01e07-Ex4-Cat and Dog Vertical Delay (unstable).bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex5-Cat and Dog Vertical Delay',
    url: '/roms/8blit-s01e07-Ex5-Cat and Dog Vertical Delay.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex6-Missiles with Color',
    url: '/roms/8blit-s01e07-Ex6-Missiles with Color.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex7-Missiles Movemen and Sizes',
    url: '/roms/8blit-s01e07-Ex7-Missiles Movemen and Sizes.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e07-Ex8-Ball Movement and Sizes',
    url: '/roms/8blit-s01e07-Ex8-Ball Movement and Sizes.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s01e08-Ex1-Sprite',
    url: '/roms/8blit-s01e08-Ex1-Sprite.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e01-Ex1-Base Frame',
    url: '/roms/8blit-s02e01-Ex1-Base Frame.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e01-Ex2-Pointer',
    url: '/roms/8blit-s02e01-Ex2-Pointer.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e01-Ex3-Pointer Two Frame Flip',
    url: '/roms/8blit-s02e01-Ex3-Pointer Two Frame Flip.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e01-Ex4-Table of Pointers',
    url: '/roms/8blit-s02e01-Ex4-Table of Pointers.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e01-Ex5-Two Ghost Different Speeds',
    url: '/roms/8blit-s02e01-Ex5-Two Ghost Different Speeds.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e02-Switches',
    url: '/roms/8blit-s02e02-Switches.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e03-Stable Screen - No Timer',
    url: '/roms/8blit-s02e03-Stable Screen - No Timer.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e03-Stable Screen - With Timer',
    url: '/roms/8blit-s02e03-Stable Screen - With Timer.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e04-Ex1-1LK One Player',
    url: '/roms/8blit-s02e04-Ex1-1LK One Player.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e04-Ex2-2LK Two Player Color',
    url: '/roms/8blit-s02e04-Ex2-2LK Two Player Color.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s02e05-Ex1-Collision Detection-Bouncing Ball',
    url: '/roms/8blit-s02e05-Ex1-Collision Detection-Bouncing Ball.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s03e01-Ex1-Randomness',
    url: '/roms/8blit-s03e01-Ex1-Randomness.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s03e02-Ex1-Sound 1 tone 1 channel',
    url: '/roms/8blit-s03e02-Ex1-Sound 1 tone 1 channel.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s03e02-Ex2-Sound multi tone 2 channel',
    url: '/roms/8blit-s03e02-Ex2-Sound multi tone 2 channel.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s03e04-Regions-before-1',
    url: '/roms/8blit-s03e04-Regions-before-1.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s03e04-Regions-before-2',
    url: '/roms/8blit-s03e04-Regions-before-2.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s03e04-Regions-final',
    url: '/roms/8blit-s03e04-Regions-final.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e01-Purrballs',
    url: '/roms/8blit-s04e01-Purrballs.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e01-Score-One-Digit',
    url: '/roms/8blit-s04e01-Score-One-Digit.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e01-Score-Two-Digits',
    url: '/roms/8blit-s04e01-Score-Two-Digits.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e01-Score-Two-Player-Decimal',
    url: '/roms/8blit-s04e01-Score-Two-Player-Decimal.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e01-Score-Two-Player',
    url: '/roms/8blit-s04e01-Score-Two-Player.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e02-Paddle-Controllers-Purrballs',
    url: '/roms/8blit-s04e02-Paddle-Controllers-Purrballs.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-s04e02-PaddleValues',
    url: '/roms/8blit-s04e02-PaddleValues.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-specials-01-125th-Subscriber',
    url: '/roms/8blit-specials-01-125th-Subscriber.bin',
    start_addr: 0xf000,
    info_url: '',
  },
  {
    name: '8blit-specials-03-christmas2022',
    url: '/roms/8blit-specials-03-christmas2022.bin',
    start_addr: 0xf000,
    info_url: '',
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
