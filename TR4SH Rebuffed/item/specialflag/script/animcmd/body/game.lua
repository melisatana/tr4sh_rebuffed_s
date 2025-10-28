game_Throw = function ()
   if sv_animcmd.is_excute() then
        sv_animcmd.ATTACK_FP(0, 0, 0x031ED91FCA, 7, 45, 80, 0, 50, 5, 0, 0, 0, 88148215354, 0, 1, 1, true, true, 0, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_PUNCH, COLLISION_SITUATION_MASK_GA, false, ATTACK_REGION_OBJECT, COLLISION_CATEGORY_MASK_NO_ITEM_STAGE, false, COLLISION_PART_MASK_ALL, false, true, true, false, 90, false, false, ATTACK_LR_CHECK_SPEED, false, false, false, false, false, COLLISION_SHAPE_TYPE_SPHERE)
    end
    if sv_animcmd.is_excute() then
        AttackModule.enable_safe_pos()
    end
    return 
end

game_Hoist = function ()
    sv_animcmd.frame(140)
    if sv_animcmd.is_excute() then
        sv_animcmd.ATTACK_FP(0, 0, 0x031ED91FCA, 21, 0, 420, 0, 69, 14, 0, 0, 0, 0x1474a84539, 0, 1, 1, true, true, 0, ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_BAT, COLLISION_SITUATION_MASK_GA, false, ATTACK_REGION_OBJECT, COLLISION_CATEGORY_MASK_NO_ITEM_STAGE, false, COLLISION_PART_MASK_ALL, false, true, true, false, 90, false, false, ATTACK_LR_CHECK_SPEED, false, true, false, false, false, COLLISION_SHAPE_TYPE_SPHERE)
    end
    if sv_animcmd.is_excute() then
        AttackModule.enable_safe_pos()
    end
    return 
end

return 
