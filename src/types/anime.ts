// 首先定义类型
export interface Anime {
    id: number
    name: string
    imageUrl: string
    status: 'ongoing' | 'completed' | 'upcoming'
}

export interface AnimeResponse {
    items: Anime[]
    total: number
}