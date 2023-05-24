import axios from 'axios';    
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import { getAlltimeLeaderboard, getCurrentLeaderboard } from '../../apis/leaderboard_apis';

export const load : () => Promise<{
    alltime: AlltimeLeaderboard,
    current: DayLeaderboard
}>  = async () => {
    const alltime = await getAlltimeLeaderboard();
    const current = await getCurrentLeaderboard();


    return {
        alltime,
        current
    };
}