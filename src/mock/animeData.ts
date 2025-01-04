import type { Anime } from '../types/anime.ts'

// Mock 数据
export const mockAnimeList: Anime[] = [
    {
        id: 1,
        name: '进击的巨人 最终季',
        imageUrl: 'https://via.placeholder.com/300x400',
        status: 'completed'
    },
    {
        id: 2,
        name: '间谍过家家',
        imageUrl: 'https://via.placeholder.com/300x400',
        status: 'ongoing'
    },
    {
        id: 3,
        name: '咒术回战 第二季',
        imageUrl: 'https://via.placeholder.com/300x400',
        status: 'ongoing'
    },
    {
        id: 4,
        name: '海贼王',
        imageUrl: 'https://via.placeholder.com/300x400',
        status: 'ongoing'
    },
    {
        id: 5,
        name: '鬼灭之刃 刀匠村篇',
        imageUrl: 'https://via.placeholder.com/300x400',
        status: 'completed'
    },
    {
        id: 6,
        name: '死神 千年血战篇',
        imageUrl: 'https://via.placeholder.com/300x400',
        status: 'upcoming'
    }
    // 可以继续添加更多数据...
]