pub(crate) const CONFIG_TOML: &str = r#"
# 频道过滤
[channel]
# 频道白名单（只允许）
white = []

# 频道黑名单（排除）
black = ["123456789"]

# 插件配置
[plugin]
# 仅 @ 响应
only_at = false

# 指令前缀
# 示例：["!", "。"]
prefixes = []

# 回复时 @
is_at = true

# 回复时引用
is_quote = false

# 神断网区域
# 可选："Jp", "En", "Cn", "Kr", "Th"
domain = "Cn"

# `随机神断` 时提示命令
random_return_command = true

# 排行榜最多显示
rank_max = 30
"#;

pub(crate) const COMMANDS_TOML: &str = r#"
[[command]]
function = "插件指令列表"
commands = ["神断帮助"]

[[command]]
function = "添加神断命令"
commands = ["添加神断"]

[[command]]
function = "删除神断命令"
commands = ["删除神断"]

[[command]]
function = "随机神断命令"
commands = ["随机神断"]

[[command]]
function = "神断命令列表"
commands = ["神断列表"]

[[command]]
function = "设置神断模式"
commands = ["设置神断"]

[[command]]
function = "修改神断命令"
commands = ["修改神断"]

[[command]]
function = "查看用户神断次数"
commands = ["用户次数"]

[[command]]
function = "用户神断次数排行榜"
commands = ["用户排行榜"]

[[command]]
function = "查看神断信息"
commands = ["查看神断"]

[[command]]
function = "神断被触发次数排行榜"
commands = ["神断次数"]

[[command]]
function = "模糊查找神断命令"
commands = ["查找神断"]
"#;

pub(crate) const USER_DATA_TOML: &str = r#"
[[user]]
id = "123456"
name = "神秘人"
count = 0
"#;

pub(crate) const SHINDANS_TOML: &str = r###"
[[shindan]]
id = "1116350"
title = "占卜你的○○度2"
description = "案例在一览表中输入 0～100 的数值，可以制作出用于“占卜出你的○○度”的小占卜"
command = "0-100"
mode = "image"
count = 0

[[shindan]]
id = "1185144"
title = "重生之我在48系做偶像"
description = "*比较符合snh生态"
command = "48系偶像"
mode = "image"
count = 0

[[shindan]]
id = "866449"
title = "测一测你的ABO性别与信息素"
description = "测试你的ABO性别和信息素的味道"
command = "ABO"
mode = "image"
count = 0

[[shindan]]
id = "1037996"
title = "ABO随机人设"
description = "人设表，随时补充，可以留言，因为完全随机（其实是不会做）所以很容易出错（可以多生成几次直到满意为止）"
command = "ABO人设"
mode = "image"
count = 0

[[shindan]]
id = "1037825"
title = "专属于你的ABO信息素，快来试一试吧"
description = "完全随机，包括部分性格，喜欢的人，外貌，身份等等。。。"
command = "ABO信息素"
mode = "image"
count = 0

[[shindan]]
id = "1213984"
title = "ABO属性"
description = "如果在ABO的世界里，会是什么样的人，拥有怎样的第二性别和信息素？"
command = "ABO属性"
mode = "image"
count = 0

[[shindan]]
id = "1134607"
title = "ABO檢測設定"
description = "[創作企劃專用]Ortus公司提供的性別、第二性徵、信息素與C細胞相性診斷資訊"
command = "ABO检测"
mode = "image"
count = 0

[[shindan]]
id = "738333"
title = "ABO角色設定"
description = "角色的性別及信息素味道！"
command = "ABO角色"
mode = "image"
count = 0

[[shindan]]
id = "1142429"
title = "AI 絵画専用ガシャポン機"
description = """
嘿！不知道该输入什么tag（prompt）来进行作画吗？
那就放空大脑，来试试最紧张刺激的抽卡环节吧！每日随机咒语！！
（理论上结果里不会出现男性，但可能出现不可直视的古神）
（咒语每日更替~）"""
command = "AI绘画提示词"
mode = "image"
count = 0

[[shindan]]
id = "1152188"
title = "测测你是什么样的 BL 角色"
description = "图一乐 随时可能更新"
command = "BL角色"
mode = "image"
count = 0

[[shindan]]
id = "525431"
title = "[R-18]BL重口H產生機"
description = "[R-18]BL重口H產生機測試，請慎入"
command = "BL重口"
mode = "image"
count = 0

[[shindan]]
id = "695886"
title = "COC人物卡生成器"
description = """
适用于第六版 配合网上流传最广的那个EXCEL人物卡来填
出来的信息直接往表里填就OK"""
command = "COC人物"
mode = "image"
count = 0

[[shindan]]
id = "1106170"
title = "第七版COC人物卡"
description = "第七版COC跑团随机人物卡"
command = "COC人物卡"
mode = "image"
count = 0

[[shindan]]
id = "1168478"
title = "COJ随机里ho生成器"
description = """
如题，跑coj不如看个ho面口胡。jpg
每次占卜都会产生变化，请注意保存
有r18g的可能"""
command = "COJ"
mode = "image"
count = 0

[[shindan]]
id = "831016"
title = "一对cp的一生是什么组成的？"
description = "被概括cp一生和你是什么组成的虐到了所以我要来发糖"
command = "CP一生"
mode = "image"
count = 0

[[shindan]]
id = "829556"
title = "测试cp的一辈子"
description = "测试cp的一辈子"
command = "CP一辈子"
mode = "image"
count = 1

[[shindan]]
id = "733273"
title = "缺梗的人快進來(❁´ω`❁)"
description = """
給你三個cp梗(*´∀`*)
每天都不同的喔( •̀∀•́ )✧"""
command = "CP三梗"
mode = "image"
count = 0

[[shindan]]
id = "1055920"
title = "适合cp/你推H时的梗"
description = """
刺激一下h灵感
（地点+时间/事件+play）这个样子"""
command = "CP三梗H"
mode = "image"
count = 0

[[shindan]]
id = "1187782"
title = "用名字生成人设和梗"
description = "用名字生成人设和梗"
command = "CP与梗"
mode = "image"
count = 0

[[shindan]]
id = "701727"
title = "圖/文。三梗系列"
description = """
輸入CP名稱，
給你三個主題去發揮！"""
command = "CP主题"
mode = "image"
count = 0

[[shindan]]
id = "829712"
title = "cp之间发生过什么"
description = "测一测你的cp曾经发生过什么（结果每天都会变喔）"
command = "CP之间"
mode = "image"
count = 0

[[shindan]]
id = "807196"
title = "每日CP Paro題材"
description = "輸入喜愛的CP，一起來創作吧～！"
command = "CP互动"
mode = "image"
count = 0

[[shindan]]
id = "1101491"
title = "你CP的关系"
description = "无聊产物 可代餐"
command = "CP关系"
mode = "image"
count = 0

[[shindan]]
id = "1098005"
title = "你CP的关键词"
description = "无聊产物"
command = "CP关键词"
mode = "image"
count = 0

[[shindan]]
id = "1161220"
title = "你/你的CP的关键词"
description = "四个关键词，拿去代餐"
command = "CP关键词2"
mode = "image"
count = 0

[[shindan]]
id = "829935"
title = "CP的命运"
description = "测一测CP的命运吧。"
command = "CP命运"
mode = "image"
count = 0

[[shindan]]
id = "1185768"
title = "猜猜我cp在干嘛"
description = "给自己写文or画画or找乐子"
command = "CP在干嘛"
mode = "image"
count = 0

[[shindan]]
id = "1164243"
title = "你cp的意难平"
description = "随便做的，并不准的测试"
command = "CP意难平"
mode = "image"
count = 0

[[shindan]]
id = "1130619"
title = "你CP的故事里..."
description = """
❤请输入你CP名❤

可以把它当作平行时空或者现实世界或者你所看不到的地方的故事。。。

*纯属娱乐 结果随机 切勿当真*
*供嗑药鸡代餐/写文灵感用*
*///句子有原创/摘录/歌词/歌名/台词等///*
*意味不明警告 非HE结果警告 玻璃心勿进*
*不准就不准嘛 不要攻击作者 作者很脆弱。。。*

*有一点点我之前做过的测试的炒冷饭*"""
command = "CP故事"
mode = "image"
count = 1

[[shindan]]
id = "829767"
title = "每日cp限定开头和限定结尾"
description = "会持续更新，给太太们一些想法啦，输入cp名就ok"
command = "CP故事2"
mode = "image"
count = 0

[[shindan]]
id = "1178875"
title = "你CP的故事里...（更新版）"
description = """
❤请输入你CP名❤

可以把它当作平行时空或者现实世界或者你所看不到的地方的故事。。。

测过旧版的觉得不准，或者想有别的结果也可以来试试。

*纯属娱乐 结果随机 切勿当真*
*供嗑药鸡代餐/写文灵感用*
*///句子有原创/摘录/歌词/歌名/台词等///*
*意味不明警告 非HE结果警告 玻璃心勿进*
*不准就不准嘛 不要攻击作者 作者很脆弱。。。*

*有一点点我之前做过的测试的炒冷饭*"""
command = "CP故事3"
mode = "image"
count = 0

[[shindan]]
id = "1190503"
title = "CP文梗（自用）"
description = "没写作灵感的时候随机一下💖"
command = "CP文梗"
mode = "image"
count = 0

[[shindan]]
id = "873026"
title = "属于CP的三句歌词"
description = "基于我歌单里的歌所组成的测试"
command = "CP歌词"
mode = "image"
count = 0

[[shindan]]
id = "1164263"
title = "你cp的甜饼日常"
description = "随便做的，刀子吃多了被强烈要求来点甜的。只适合甜甜纯爱"
command = "CP甜饼"
mode = "image"
count = 0

[[shindan]]
id = "1155984"
title = "搞cp用"
description = "自用产品生成器而已，应该都是be捏。"
command = "CP生成"
mode = "image"
count = 0

[[shindan]]
id = "835973"
title = "属于他（她）们的幸福结局"
description = "来测测cp们的happy ending是什么样的吧！没有刀，只有糖～"
command = "CP结局"
mode = "image"
count = 0

[[shindan]]
id = "1194680"
title = "基于emoji的CP出题器"
description = "结果每次变化，请输入cp名"
command = "CP表情"
mode = "image"
count = 0

[[shindan]]
id = "1160415"
title = "今天你cp是什么身份"
description = """
今天你cp会是以何种身份相遇的呢
（没灵感做出来自用的，慢慢更新）"""
command = "CP身份"
mode = "image"
count = 0

[[shindan]]
id = "1198474"
title = "cp道具赛"
description = "来点道具场景的灵感"
command = "CP道具"
mode = "image"
count = 0

[[shindan]]
id = "807174"
title = "每日CP互動題材"
description = """
輸入喜愛的CP，一起來創作吧～
是甜是虐任由發揮！"""
command = "CP题材"
mode = "image"
count = 0

[[shindan]]
id = "928742"
title = "你家CP要不要來點題材？"
description = "抽到對話/短打請自行調整適合該CP的語氣/文風！很雜，不適合大部分CP（欸"
command = "CP题材2"
mode = "image"
count = 0

[[shindan]]
id = "1118780"
title = "kpop嗑药鸡cp黄文生成器"
description = "如题"
command = "CP黄文生成"
mode = "image"
count = 0

[[shindan]]
id = "1204473"
title = "看看你是CV服里的谁"
description = "玩服务器玩的"
command = "CV服"
mode = "image"
count = 0

[[shindan]]
id = "1120226"
title = "群黑鬼的DND因子大抽奖（入门难度）"
description = "比简单还简单大概就是这样？"
command = "DND抽奖"
mode = "image"
count = 0

[[shindan]]
id = "1120199"
title = "群黑鬼的DND因子大抽奖（困难难度）"
description = "迎接DM的恶意吧"
command = "DND抽奖2"
mode = "image"
count = 0

[[shindan]]
id = "1120198"
title = "群黑鬼的DND因子大抽奖（标准难度）"
description = "总之这是难度还说的过去的一组"
command = "DND抽奖3"
mode = "image"
count = 0

[[shindan]]
id = "1120197"
title = "群黑鬼的DND因子大抽奖（简单难度）"
description = "总之这是最简单的一组"
command = "DND抽奖4"
mode = "image"
count = 0

[[shindan]]
id = "1204609"
title = "占卜你的Dota2英雄"
description = "由于网站限制，没办法做得太精细，因为单次占卜能显示的字数有限所以技能留到下次"
command = "DOTA2英雄"
mode = "image"
count = 0

[[shindan]]
id = "1164788"
title = "fe十连抽！【姜安生】"
description = "来呀烬粉"
command = "FE十连"
mode = "image"
count = 0

[[shindan]]
id = "1164769"
title = "FE十连抽_【宋栖ver】"
description = "来一发吗亲爱的烬粉"
command = "FE十连2"
mode = "image"
count = 0

[[shindan]]
id = "1164767"
title = "FE十连抽【宋栖ver】"
description = "来一发吗亲爱的烬粉"
command = "FE十连3"
mode = "image"
count = 0

[[shindan]]
id = "1164760"
title = "fe宋栖酱10连抽。。"
description = "这是通过使用 “一览表的细分编号功能” 制作 10 连抽的范例。"
command = "FE十连4"
mode = "image"
count = 0

[[shindan]]
id = "1164758"
title = "fe宋栖 10连抽"
description = "这是通过使用 “一览表的细分编号功能” 制作 10 连抽的范例。"
command = "FE十连5"
mode = "image"
count = 0

[[shindan]]
id = "1164756"
title = "fe十连抽！【宋栖】"
description = "来一发"
command = "FE十连6"
mode = "image"
count = 0

[[shindan]]
id = "1138528"
title = "【FF14】测测你的rp人设"
description = "还花钱买啥人设啊！"
command = "FF14人设"
mode = "image"
count = 0

[[shindan]]
id = "1140848"
title = "ff14冒险者登记！"
description = """
抽选一位冒险者行会登记的冒险者！包括一个随机的基础面板和性格小菜单
注：
1.不会出现主线经历冒险者才能学会的职业/特职：白魔法师 黑魔法师 学者，目前只有基础职业
2.因为冒险者行会目前在三主城，所以地域性职业（远东近战，舞者，绝枪，伊修加德等）不列入其中
3.可能出现的种族分布地和职业学习的冲突和语句不通顺 请多多包容呜呜！"""
command = "FF14冒险者"
mode = "image"
count = 0

[[shindan]]
id = "1141953"
title = "FF14同人梗生成器（截至6.1版本）"
description = """
·请输入名字，例如：oc名、光之战士、NPC名……
·NPC名单和梗池持续更新，含🚫非全龄🚫梗、paro梗、烂梗等等，大概率出现让人眼前一黑的结果。
·每次占卜结果都不同，实在roll不到心选/不想和NPC搞的话请自由替换。"""
command = "FF14同人梗"
mode = "image"
count = 0

[[shindan]]
id = "1191716"
title = "在FGO成为了这样的从者"
description = """
我爸爸在lxs工作，小心你被他做成这样的从者.JPG
变量槽位不够，精简了一下，连宝具卡色都没有的阉割版本。
宝具名按照『A，BC』格式随机（例：日轮啊，顺从死亡）。"""
command = "FGO从者"
mode = "image"
count = 0

[[shindan]]
id = "1157240"
title = "fgo从者化"
description = "职介+稀有+属性+六维+技能+宝具"
command = "FGO从者化"
mode = "image"
count = 0

[[shindan]]
id = "1129774"
title = "重生之假如我成为了FGO英灵"
description = """
如同雷同，纯属巧合
仅供娱乐，切勿参考"""
command = "FGO英灵"
mode = "image"
count = 0

[[shindan]]
id = "1128812"
title = "未來那一天在HPFL..."
description = """
小小一個舉動可能會造成大大的不同
身為造物羊更是如此
快來看看造物羊的甚麼舉動會影響到你吧？"""
command = "HPFL"
mode = "image"
count = 0

[[shindan]]
id = "863622"
title = "你的cp在H时是怎样呻吟的"
description = "有点小污污啦🙈"
command = "H呻吟"
mode = "image"
count = 0

[[shindan]]
id = "42890"
title = "【老湿测试】H的时候的呻吟"
description = "继续娱乐"
command = "H呻吟续"
mode = "image"
count = 0

[[shindan]]
id = "1043561"
title = "絕頂管理的淫慾地下城 byあかいししろいし（艾奈茲漢化）"
description = """
對象（♀）會長時間被觸手等持續強制高潮，或是相反地不被允許高潮。因為太喜歡mitake2359的工口陷阱地下城(a/570937)了，所以作為致敬便做了這個。完全是自己的興趣。基本上是照自己那執著的性癖製作。還有在這個地下城裡不管過了多少時間都不會老也不會感到肚子餓。字數也很多。
只有【角色名】也可以，我想寫成【角色名】○○層應該也會蠻有趣的。加上HP之類的就看各位的喜好了。如果可以畫成畫的話還請務必畫畫看。那麼就這樣！
(ver1.05)"""
command = "H地下城"
mode = "image"
count = 0

[[shindan]]
id = "1090382"
title = "H梗生成器"
description = """
不断更新中...逻辑可能有点乱抱歉💦重新测就好啦
输入[cp名]
输出格式为[世界观]+[场景]+[人设]+[play]+[名台词]
*含acg捏他"""
command = "H梗"
mode = "image"
count = 0

[[shindan]]
id = "1130264"
title = "【牙獸種篇】慘遭H的魔物獵人"
description = "狩獵失敗就當場H的魔物獵人。男女皆可用。牙獸種篇。不包含邊境魔物。"
command = "H魔物猎人"
mode = "image"
count = 0

[[shindan]]
id = "1148399"
title = "欢迎来到IAM!"
description = "「仅供娱乐，任何问题去找顾烬」"
command = "IAM"
mode = "image"
count = 0

[[shindan]]
id = "16051"
title = "小JJ长度预测"
description = "123"
command = "JJ长度"
mode = "image"
count = 0

[[shindan]]
id = "1000666"
title = "JOJO里谁想和你做爱？"
description = "www"
command = "JOJO做爱"
mode = "image"
count = 0

[[shindan]]
id = "1075343"
title = "大概是最JOJO的替身测试"
description = """
替身名都有具体的乐队名/专辑名/曲名出处。
外加颜色、形状、能力属性（听上去像宝可梦）。
内含不少彩蛋，是欧洲人的话测试几次应该能发现。
数值对应：
1 = E
2 = D
3 = C
4 = B
5 = A"""
command = "JOJO替身"
mode = "image"
count = 0

[[shindan]]
id = "965601"
title = "穿越到JOJO世界中谁会爱上你呢？你最终又会和谁在一起呢？"
description = "梦向噢"
command = "JOJO老婆"
mode = "image"
count = 0

[[shindan]]
id = "1115946"
title = "假如让你打造一个LDH男团："
description = """
全部结果为随机生成，作者不负任何责任，有侵权会删除｜
测试目前成员包涵本部/二团/jsb/gene/芬达/浪配/bbz/di
｜其中为了区分方便，bbz一律写为per，di一律写成vo"""
command = "LDH男团"
mode = "image"
count = 0

[[shindan]]
id = "1188647"
title = "pxxn的lol狼人杀"
description = "每次占卜结果会自动改变"
command = "LOL狼人杀"
mode = "image"
count = 1

[[shindan]]
id = "1157324"
title = "隨機mbti和九型"
description = "自用"
command = "MBTI和九型"
mode = "image"
count = 0

[[shindan]]
id = "1162074"
title = "MGS出行"
description = "MGS群友自用"
command = "MGS出行"
mode = "image"
count = 0

[[shindan]]
id = "1106594"
title = "【第一次试做】oc元素关键词生成（要素过多）"
description = """
大概就是，做来生成好多好多小马这样
（每占卜一次就会变，所以是量产机这样）
经常不好使 觉得不喜欢就重新来 总会有几个能行
（一部分颜色是从潘彤抱的，色号错了别打我）
占卜事例（方便大家明白有哪些内容）
●云尼拿绿色的茶杯
●再搭配一点深紫色，还有绀色
●也许会成为一位作曲家
●百里香
●天文台，山羊，还有提拉米苏
●有着文艺复兴风格，ta看起来有点悲哀
●如果是小马的话，应该是飞马
●如果到霍格沃茨的话，会分到斯莱特林
●话说，有没有闻到水果的味道？"""
command = "OC元素关键词"
mode = "image"
count = 0

[[shindan]]
id = "1075782"
title = "OC十连池"
description = "[自娱自乐向]十连保底SR一枚，输入单人orCP都可以"
command = "OC十连"
mode = "image"
count = 0

[[shindan]]
id = "1139005"
title = "oc单up池抽抽乐"
description = """
自娱自乐产物
每日结果不同"""
command = "OC单UP池"
mode = "image"
count = 0

[[shindan]]
id = "1139004"
title = "oc单up池十连"
description = """
自娱自乐产物
每日结果不同"""
command = "OC单UP池2"
mode = "image"
count = 0

[[shindan]]
id = "1156117"
title = "关于oc的印象物"
description = "可以用来完善设定什么的…有可能会有重复的不好意思！"
command = "OC印象物"
mode = "image"
count = 1

[[shindan]]
id = "1057774"
title = '☆·":*人設換裝*:"·☆'
description = "給人設或OC換衣服叭(*•̀ᴗ•́*)و ̑"
command = "OC换装"
mode = "image"
count = 0

[[shindan]]
id = "1116736"
title = "♣OC生成器（Demo版）♣"
description = "结果还比较少，试验性质，有空再更新。♣结果显示：发色，瞳色，肤色，体质，职业，性格，星座，擅长的，喜欢的，讨厌的♣"
command = "OC生成"
mode = "image"
count = 0

[[shindan]]
id = "1174414"
title = "奶茶的人设生成器"
description = "好好玩吧"
command = "OC生成10"
mode = "image"
count = 0

[[shindan]]
id = "1155706"
title = "创作一个oc"
description = "装饰性 元素构成的角色"
command = "OC生成11"
mode = "image"
count = 0

[[shindan]]
id = "1140990"
title = "OC Creator(Based on the PES:B Universe) "
description = "自用"
command = "OC生成2"
mode = "image"
count = 0

[[shindan]]
id = "1215134"
title = "OC生成器（颜色可视化ver）"
description = """
主攻外貌方面的颜色可视化的生成器
前作：ID:1214005
前前作：ID:1213942"""
command = "OC生成4"
mode = "image"
count = 0

[[shindan]]
id = "1153293"
title = "oc生成器/oc creator"
description = '''
!!! There are non-human skin colors !!!
Any skin color with "（异色皮）" is non-human
包含发色、瞳色、其他外貌特征、喜欢的、讨厌的、身高、肤色、星座
For the English version pls go to ID:1153609

2023/1/19 更新了，加了肤色和其他词语
2023/3/28 更新了，加了性格关键词'''
command = "OC生成5"
mode = "image"
count = 0

[[shindan]]
id = "1138362"
title = "OC生成器✨"
description = """
一个简单的oc元素生成器！
想不到元素的时候或许可以试试看。"""
command = "OC生成6"
mode = "image"
count = 0

[[shindan]]
id = "1213942"
title = "我也要搞oc"
description = """
oc，爽！
部分参考自Yannie的♣OC生成器（Demo版）♣（ID:1116736）
仅用于提供灵感，建议修改后使用，请勿商用。
我知道有BUG（比如对自己过敏）（划掉），但我不想修。
最后：为什么只能有十个一览表啊啊啊啊啊啊！！！！
不过幸好我会用随机数（"""
command = "OC生成7"
mode = "image"
count = 0

[[shindan]]
id = "1191235"
title = "【自用】oc创作"
description = "发色，瞳色，性别，身高，性格，种族"
command = "OC生成8"
mode = "image"
count = 0

[[shindan]]
id = "1188466"
title = "oc生成器自用世界观版"
description = "自用的所以只写了自己想写的，想到想放里面的就会更新一下，几率有时候也会改，只适用于偏西幻类型的架空世界观，虽然写的都是自己世界观的种族，没有的种族写什么都无所谓，名字建议用中性化的或者随机完了再起名，英文缩写也可以用来起名不过可能会有怪东西被随机出来而且只有两个字母"
command = "OC生成9"
mode = "image"
count = 0

[[shindan]]
id = "1165958"
title = "来给你的oc一套衣服吧！"
description = """
（我不想再全家上下二十口只有两件衣服了啊啊啊！
（我放弃搞随机颜色了，反正站内也已经有了，我就不做了）"""
command = "OC衣服"
mode = "image"
count = 0

[[shindan]]
id = "1117103"
title = "你在女神异闻录4中的迷宫"
description = "这个测试包含迷宫的风格以及形成迷宫的原因，会有很牙白的内容，准不准我说不定:/"
command = "P4迷宫"
mode = "image"
count = 0

[[shindan]]
id = "1128682"
title = "随机颜色（RGB）"
description = "随意配色"
command = "RGB"
mode = "image"
count = 0

[[shindan]]
id = "741327"
title = "你在RPG遊戲中的正副職業"
description = "你在RPG遊戲中的正副職業"
command = "RPG职业"
mode = "image"
count = 0

[[shindan]]
id = "28414"
title = "SM测试"
description = "测测你是S还是M"
command = "SM"
mode = "image"
count = 0

[[shindan]]
id = "162994"
title = "你作為Servant的屬性"
description = "測試你在fate中會成為哪個英靈，擁有怎樣的結局"
command = "Servant"
mode = "image"
count = 0

[[shindan]]
id = "1153659"
title = "Shanthi家OC角色发生器1.0"
description = "只是一个娱乐性生成器！应用世界观为“主世界：剧场”。"
command = "Shanthi"
mode = "image"
count = 0

[[shindan]]
id = "1204675"
title = "生成你的UA卡牌效果！"
description = "输入名称生成你的UA卡牌吧！"
command = "UA卡牌"
mode = "image"
count = 0

[[shindan]]
id = "1155821"
title = "txt第六名成员生成器"
description = """
aka打乱成员的数据后会生成什么样的孩子
又aka五穆棱乱交后代生成器（
内含花名，目前只是粗制版，会持续补充更新"""
command = "aka乱交成员"
mode = "image"
count = 0

[[shindan]]
id = "1153267"
title = "arma3在基地重生"
description = """
arma3
最后更新2023.1.18"""
command = "arma3在基地重生"
mode = "image"
count = 0

[[shindan]]
id = "1136609"
title = "你是homolism吗"
description = "你就是个homo罢(悲)"
command = "homo指数"
mode = "image"
count = 0

[[shindan]]
id = "1135995"
title = "【自用】脑谈台词生成器"
description = "正午箭袋提供射手英雄所需的攻击力和攻击速度，并增强他们的PVE能力，且合成路线平滑，是adc理想的前期装备。同时，它也是合成ad神话装备狂风之力、海妖杀手和不朽盾弓的必要构件。中文名正午箭袋外文名Noonquiver物品属性唯一：神话构件+30攻击力+15%攻击速度精密：攻击会对小兵和野怪额外造成20物理伤害"
command = "nt台词"
mode = "image"
count = 0

[[shindan]]
id = "799899"
title = "CP向一句話創作圖文"
description = "可以輸入CP或任意的名字，隨機抽一句話，就只是一句話，甜虐操之您手，語氣用詞等請自由調整"
command = "一句话"
mode = "image"
count = 1

[[shindan]]
id = "1140051"
title = "生成你的一句话故事"
description = "输入你的假名（不要真名，因为可能会随机到坏选项）"
command = "一句话故事"
mode = "image"
count = 0

[[shindan]]
id = "1190840"
title = "一方苗圃"
description = "没有了，什么都没有了，嘿，带上这孩子吧，你是“有色的”，对吧？"
command = "一方苗圃"
mode = "image"
count = 0

[[shindan]]
id = "761933"
title = "七宗罪程度測量"
description = "捏塑角色惡習時使用，圓餅圖計算"
command = "七宗罪"
mode = "image"
count = 0

[[shindan]]
id = "1134084"
title = "万圣节的他们变装成了……"
description = "产粮随机挑战！"
command = "万圣节变装"
mode = "image"
count = 0

[[shindan]]
id = "1183660"
title = "在角落遇到的三个字。"
description = "对似乎寓示着命运的奇妙词语，该怀着什么样的心情去把结果揭开？"
command = "三个字"
mode = "image"
count = 1

[[shindan]]
id = "898330"
title = "CP每日三个关键词"
description = "糖刀参半，想不出梗的小伙伴可以试试呀"
command = "三关键词"
mode = "image"
count = 0

[[shindan]]
id = "1134777"
title = "三句有缘的小众古诗"
description = "点进来就是有缘分，看看你的名字占卜出来的古诗吧"
command = "三句古诗"
mode = "image"
count = 0

[[shindan]]
id = "1125024"
title = "那么下周的晚饭"
description = "该案例利用“一览表的细分编号功能”，只使用一个一览表显示多个值，然后在一览表选项中将使用过的值设置为“删除（一览表 1 的所有相同值）”，使同样的值不会重复显示。"
command = "下周晚饭"
mode = "image"
count = 0

[[shindan]]
id = "1164265"
title = "不健康恋爱关系"
description = "随便做的，没有什么意义"
command = "不健康恋爱关键词"
mode = "image"
count = 0

[[shindan]]
id = "1215542"
title = "不存在的小县城"
description = "生成一个让人一听会觉得是几百公里外真有这么个地方的地方"
command = "不存在的县城"
mode = "image"
count = 0

[[shindan]]
id = "28733"
title = "哪裡不對勁"
description = "哪裡不對勁怎麼辦"
command = "不对劲"
mode = "image"
count = 0

[[shindan]]
id = "832081"
title = "製作你的專屬魔藥☆"
description = "也可以算是一個畫畫的腦洞生成器？ww"
command = "专属魔药"
mode = "image"
count = 0

[[shindan]]
id = "1204276"
title = "世警大乐透，有命你就来"
description = "输入ID获取您的报告"
command = "世警大乐透"
mode = "image"
count = 0

[[shindan]]
id = "1153313"
title = "丘比特之箭"
description = "做饭的时候心动的点最难写故作此诊断。（内容不够充实缓慢更新中）"
command = "丘比特之箭"
mode = "image"
count = 0

[[shindan]]
id = "1161317"
title = "尝试制作的东方风格称号生成器"
description = """
大部分词汇都是从原作扒下来的然后自己写了一点💦💦因为只是想做着试试其实没什么zun味，请纯粹作为乐子看就好，非常抱歉……
也许后面会再加入一些词语，如果可以编辑或者我想得起来的话……"""
command = "东方风格称号"
mode = "image"
count = 0

[[shindan]]
id = "1089994"
title = "个人属性"
description = "喵叽？（⑨⑨⑨⑨⑨⑨⑨⑨⑨）"
command = "个人属性"
mode = "image"
count = 0

[[shindan]]
id = "790697"
title = "奇妙的中二称号生成器"
description = "快乐称号"
command = "中二称号"
mode = "image"
count = 0

[[shindan]]
id = "162202"
title = "于是，一场中国的圣杯战争开始了"
description = "这是一场来自中国的圣杯战争"
command = "中国圣杯战争"
mode = "image"
count = 0

[[shindan]]
id = "21973"
title = "我的主人v.1.0"
description = "身為一名女僕或是管家,你/妳的主人會是個什麼樣的人呢？"
command = "主人"
mode = "image"
count = 0

[[shindan]]
id = "1189474"
title = "以你为主题的香水"
description = """
写了很多很多成分！一千字无重复项！平均每个位置有三十多个不重复项！快来玩！🥺
会有非具象的结果和偏负面的结果
内含有点多的电影名字，少量歌名，少量书名，以及极少量的恶趣味
也许可以当做什么电影推荐器（笑）

每次占卜的结果都会变 建议截图保存……"""
command = "主题香水"
mode = "image"
count = 0

[[shindan]]
id = "731066"
title = "你的人格特質分布（九型人格）"
description = "輸入名字診斷看看你的人格特質分布吧！（輸入角色名稱也OK）"
command = "九型人格"
mode = "image"
count = 0

[[shindan]]
id = "851659"
title = "indie二次元人设生成器"
description = """
天呐怎么会有这么不适合二次元化的测试！
【每日更新】"""
command = "二次元人设生成"
mode = "image"
count = 0

[[shindan]]
id = "210287"
title = "测测乃的二次元少女形象（加强版）"
description = "测测乃的二次元少女形象的加强版"
command = "二次元少女"
mode = "image"
count = 0

[[shindan]]
id = "288809"
title = "你的二次元男性設定"
description = "男女適用~"
command = "二次元男性"
mode = "image"
count = 0

[[shindan]]
id = "22206"
title = "二次元的你是？"
description = "測試看看你的二次元形象"
command = "二次元的我"
mode = "image"
count = 0

[[shindan]]
id = "1136285"
title = "你的二次元老婆是"
description = "抽老婆"
command = "二次元老婆"
mode = "image"
count = 0

[[shindan]]
id = "1148696"
title = "于生命终焉"
description = "特别特别土，可以代自己、oc或者推し～"
command = "于生命终焉"
mode = "image"
count = 0

[[shindan]]
id = "383328"
title = "互動命題(*´ﾟ∀ﾟ)从(ﾟωﾟ｀*)"
description = "適用於微博/推特/噗浪的互動遊戲 輸入角色或CP名(還可以+地點名)進行測試(含有小糟糕PLAY)玩不起來也別在意哦(ゝω・)キラッ☆ CP腦洞關鍵字測試ID:384482"
command = "互动命题"
mode = "image"
count = 0

[[shindan]]
id = "1123549"
title = "五覺剝奪世界paro"
description = "這個世界的人一出生就會被剝奪五覺的其中一個，在這個情況下必須遇到自己一生中的命運伴侶，才能擁有失去的那一個感知。"
command = "五觉剥夺世界"
mode = "image"
count = 0

[[shindan]]
id = "1214661"
title = "限定五个词条来写一篇文吧(o^^o)♪"
description = """
自用，没灵感没脑洞的时候会过来抽几个词条来写。可以无限重复抽词条，只要刷新就好～（有概率掉落R18词条）
（只要脑洞大，不小心抽到相悖的词条，反而会变得更有趣了(ﾟωﾟ)"""
command = "五词条"
mode = "image"
count = 0

[[shindan]]
id = "1214543"
title = "人偶生成器<3"
description = "喜欢我永远的后日谈吗"
command = "人偶生成"
mode = "image"
count = 0

[[shindan]]
id = "1183682"
title = "人偶记忆碎片"
description = "自死亡中再起的人偶啊，在那荒芜的空白中，还记得些什么呢？"
command = "人偶记忆碎片"
mode = "image"
count = 0

[[shindan]]
id = "827798"
title = "測試你的人格面具（Persona）屬性"
description = """
測試你的人格面具（Persona）的能力值。
有部分人格面具的種類是較為稀有的。"""
command = "人格面具"
mode = "image"
count = 0

[[shindan]]
id = "1162252"
title = "人物基本数值生成"
description = "不知道怎么定人物的性格和外貌？来这里用数值决定吧！"
command = "人物基本数值"
mode = "image"
count = 0

[[shindan]]
id = "1136727"
title = "TA的人生"
description = "TA的人生是什么样的（抽象式表达）"
command = "人生"
mode = "image"
count = 0

[[shindan]]
id = "917962"
title = "人设生成器"
description = "测测你的人设吧,每日变更"
command = "人设生成"
mode = "image"
count = 0

[[shindan]]
id = "976056"
title = "测试你的人设-2.0！"
description = "这是之前的测试的再制作版本，欢迎大家使用！同时，也可用来制作无限流的基础人物卡XD死法高达百中，种族（国籍）高达80种，职业有40多种，仍在不断更新"
command = "人设生成2"
mode = "image"
count = 0

[[shindan]]
id = "1137397"
title = "人設製造機"
description = "人設製造機"
command = "人设生成3"
mode = "image"
count = 0

[[shindan]]
id = "1123349"
title = "人設產生"
description = "使用三个一览表可生成较复杂文本的占卜。"
command = "人设生成4"
mode = "image"
count = 0

[[shindan]]
id = "26637"
title = "-人造少女-"
description = "您家的女兒究竟是什麼樣子的呢？"
command = "人造少女"
mode = "image"
count = 0

[[shindan]]
id = "23346"
title = "我的僕人v1.0"
description = "如果你能擁有一個常伴在身邊服侍自己的僕人,會是什麼樣子的人呢？"
command = "仆人"
mode = "image"
count = 0

[[shindan]]
id = "1150012"
title = "今天可以吃巧克力吗"
description = "甜食不能天天吃噢"
command = "今天可以吃巧克力吗"
mode = "image"
count = 0

[[shindan]]
id = "1147498"
title = "今日人品(自用)"
description = "自用"
command = "今日人品"
mode = "image"
count = 0

[[shindan]]
id = "1150537"
title = "今日任斗Main是？"
description = "占卜今天在任斗SP中，你的主力角色"
command = "今日任斗"
mode = "image"
count = 0

[[shindan]]
id = "1126051"
title = "今天的也最喜歡了 可蕾版本"
description = "以近期有出勤的為主，多使用制服照，部分人用活動照片。"
command = "今日可蕾"
mode = "image"
count = 0

[[shindan]]
id = "1062001"
title = "☆多娜多娜人材属性☆"
description = "♡~赛博朋克风的多娜多娜，快来测试你的人材属性吧~♡"
command = "今日多娜多娜人材"
mode = "image"
count = 1

[[shindan]]
id = "1200414"
title = "【自用向】今天要來點妖狐嗎"
description = "用來決定今天畫什麼皮的崽，一半以上是決平的皮，陰陽師本家的沒有全放。出於個人喜好因素，不會骰出逐月昭君和落月歸人"
command = "今日妖狐"
mode = "image"
count = 0

[[shindan]]
id = "1062077"
title = "☆宝可梦图鉴☆"
description = "如果你成为了宝可梦，会是属于什么样的存在呢？"
command = "今日宝可梦"
mode = "image"
count = 0

[[shindan]]
id = "162207"
title = "你的二次元少女化形象"
description = "来测试一下你的二次元少女化形象吧"
command = "今日少女"
mode = "image"
count = 0

[[shindan]]
id = "1152099"
title = "今天的欧派是什么样"
description = """
抽卡看自己今天会有什么欧派
最后更新时间2023.1.18"""
command = "今日欧派"
mode = "image"
count = 0

[[shindan]]
id = "1061975"
title = "画画用人设生成器"
description = "你发现了好玩的新东西 快来玩玩ﾚ(ﾟ∀ﾟ;)ﾍ=З=З=З 带颜色的人设oc生成器"
command = "今日画画人设"
mode = "image"
count = 0

[[shindan]]
id = "1168322"
title = "今天的你是什麼樣的糖果呢？"
description = "不管怎樣都很可愛！每天都有不同的味道喔。"
command = "今日糖果"
mode = "image"
count = 0

[[shindan]]
id = "1077564"
title = "今天穿的什么颜色的胖次"
description = "今天穿的什么颜色的胖次"
command = "今日胖次"
mode = "image"
count = 0

[[shindan]]
id = "1060224"
title = "生成专属于你的誓灵"
description = "根据手游方舟指令做的誓灵生成器，内容来自方舟指令wiki，不定期更新"
command = "今日誓灵"
mode = "image"
count = 0

[[shindan]]
id = "1163424"
title = "今日运势~36~"
description = "尝试做一下真的占卜【？】"
command = "今日运势"
mode = "image"
count = 15

[[shindan]]
id = "1146701"
title = "从不，很少，偶尔，总是"
description = "从不，很少，偶尔，总是"
command = "从不偶尔很少总是"
mode = "image"
count = 0

[[shindan]]
id = "1183493"
title = "你来自宇宙的何方？"
description = """
在名为漫长的宇宙中，你来自何方？
（来自文明的更新）"""
command = "从何而来"
mode = "image"
count = 0

[[shindan]]
id = "1134655"
title = "预测你的从前"
description = "男性向，古代"
command = "从前"
mode = "image"
count = 0

[[shindan]]
id = "916394"
title = "他們的結局"
description = "CP向，大概都是BE,謝謝使用此診斷，畫圖或是寫文都非常歡迎。"
command = "他们的结局"
mode = "image"
count = 0

[[shindan]]
id = "1148392"
title = "欢迎来到代号M！"
description = """
「世界构建成功」
1.0版本，欢迎来抽卡。"""
command = "代号M"
mode = "image"
count = 0

[[shindan]]
id = "1064580"
title = "♏️あなたの令呪風アイコン作るったー♋️(3画修正＆気ままに増量中)"
description = """
オリジナル二次創作の、fate令呪風アイコンを診断します。
全て3画で構成されてます。(一部消し忘れがあったので3画に直しました)
アイコンや加工もokですが、あくまで二次創作物。
ID:1064502様に影響されて作りました。こっちもぜひやってください。
第4弾→ID:1065915 (ソウルジェム画像診断)"""
command = "令咒"
mode = "image"
count = 0

[[shindan]]
id = "1148497"
title = "來生成「以諾之書」中的人物！"
description = """
隨便玩玩(o^^o) 僅供娛樂！！
每次生成的結果都不一樣，總有一款適合你。"""
command = "以诺之书角色"
mode = "image"
count = 0

[[shindan]]
id = "1127983"
title = "你的伤疤是怎么来的"
description = "输入一个人的名字，然后如题"
command = "伤疤"
mode = "image"
count = 0

[[shindan]]
id = "1214714"
title = "众所周知（伪）的二元妖精"
description = """
实际上是不为人知的
自己世界观的一个开发
另外欢迎大家玩我主页其他占！
是一个oc人、画画人大友好的占
世界观详情→https://miracle1024.wordpress.com/2024/08/03/%e4%ba%8..."""
command = "伪二元妖精"
mode = "image"
count = 0

[[shindan]]
id = "1189540"
title = "（修真oc）楠阳界•人族初始设定抽取 —（已制作完成）"
description = "来自中国的修真oc群 欢迎大家加入 QQ群：416507736"
command = "修真人族"
mode = "image"
count = 0

[[shindan]]
id = "1216717"
title = "〔修真测试〕|〔楠阳界〕•「初始设定抽取」|〔兽〕 |（未制作完成）"
description = "（未制作完成）"
command = "修真兽族"
mode = "image"
count = 0

[[shindan]]
id = "1190103"
title = "（修真oc）楠阳界•妖族初始设定抽取 —（已制作完成）"
description = "来自中国的修真oc群 欢迎大家加入 QQ群：416507736"
command = "修真妖族"
mode = "image"
count = 0

[[shindan]]
id = "1198315"
title = "测试你的 ‘修真资质’ 吧 ---- 已制作完成"
description = "来自中国的修真oc 测试器 欢迎大家测试自己的 ‘修真资质’ "
command = "修真资质"
mode = "image"
count = 0

[[shindan]]
id = "1204477"
title = "俾斯麦餐厅今日菜单"
description = "俾斯麦餐厅的俾斯麦餐厅提供五大洋各个角落的珍馐美味，自信能胜任任何宫廷宴席，是艾欧泽亚响当当的头等餐厅。来到此处的客人都会像吞噬一切的白鲸那样，把所有端上来的食物都吃的干干净净。"
command = "俾斯麦餐厅菜单"
mode = "image"
count = 0

[[shindan]]
id = "828228"
title = "假面骑士生成器"
description = "如果你是假面骑士的话……"
command = "假面骑士"
mode = "image"
count = 0

[[shindan]]
id = "1122723"
title = "某天做的夢（測試）"
description = """
做好玩的。
因為是夢所以隨便啦。"""
command = "做梦"
mode = "image"
count = 0

[[shindan]]
id = "26988"
title = "傲嬌"
description = "傲嬌程度測定"
command = "傲娇程度"
mode = "text"
count = 5

[[shindan]]
id = "1128866"
title = "随机元素盲盒"
description = "你好！你的【随机元素盲盒】已送达："
command = "元素盲盒"
mode = "image"
count = 0

[[shindan]]
id = "1208061"
title = "六月份你被表白的概率"
description = "5月份没有表白的小伙伴们不用担心。"
command = "六月份被表白概率"
mode = "image"
count = 0

[[shindan]]
id = "1134331"
title = "属于你的关键词"
description = "一些意象堆砌。可输入cp名或者角色名。可用于代餐和创作！第一次做请多关照ouo"
command = "关键词"
mode = "image"
count = 0

[[shindan]]
id = "1103980"
title = "假如你打造了一个内娱9人女团"
description = "无聊产物 纯属娱乐"
command = "内娱女团"
mode = "image"
count = 0

[[shindan]]
id = "1102872"
title = "测试你的冉群属性"
description = "你是anti吗"
command = "冉群属性"
mode = "image"
count = 0

[[shindan]]
id = "1102874"
title = "测试你的冉群属性2"
description = "你是anti吗"
command = "冉群属性2"
mode = "image"
count = 0

[[shindan]]
id = "1161235"
title = "農場生活"
description = "農場的新生活開始了"
command = "农场生活"
mode = "image"
count = 0

[[shindan]]
id = "1117355"
title = "冥界的生命履歷"
description = """
地下世界的招人單
（非現世）"""
command = "冥界生命履历"
mode = "image"
count = 0

[[shindan]]
id = "1158792"
title = "抽取创作关键词"
description = "创作关键词"
command = "创作关键词"
mode = "image"
count = 0

[[shindan]]
id = "1141770"
title = "✨創作靈感✨"
description = "可能是一個字、一句話或是一篇文章"
command = "创作灵感"
mode = "image"
count = 0

[[shindan]]
id = "1109819"
title = "创作用句2/2"
description = "觉得好玩就做了给自己用，有部分来源于歌词，如果合适的话可用作CP向！发出来让大家图个乐子，或者别的什么，随便啦，会慢慢更新的"
command = "创作用句"
mode = "image"
count = 0

[[shindan]]
id = "1135815"
title = "來創造偶像吧"
description = "我無聊想寫點什麼所以我做這個"
command = "创造偶像"
mode = "image"
count = 0

[[shindan]]
id = "829014"
title = "神明创造你的十日"
description = "从虚无中诞生的肉体，生命破土而出。神创造名为“你”的人类时，做出了什么选择？（微猎奇向）"
command = "创造我"
mode = "image"
count = 0

[[shindan]]
id = "1177226"
title = "创造故事！附带有趣背景与内心世界的角色设定"
description = """
舍弃了简单地描述角色的性格特质，而是可以生成角色的心理世界的种种关键点，可以让人一窥ta的内心。这个生成器更适合生成现实色彩的青春期角色

依赖：这个内容指的是在生活中能支持ta继续生活的某种体验，我通常从这样一个有趣的体验开始动笔、想象这个角色为什么会依赖于这样的境遇

障碍：角色内心的心结，我将角色的创伤性记忆，秘密，恐惧和讨厌的东西都归在了这一类

矛盾：角色的某种心理处境，可以看作角色的母题和需要克服的生活，熟练的作者可以按照自己的想象并忽略

家庭：帮助我们窥视角色成长中的影响因素"""
command = "创造故事"
mode = "image"
count = 0

[[shindan]]
id = "1174420"
title = "神明制作你的四样东西"
description = "每一个人都有属于自己的神明，而它在制作你的时候会使用四样物品。"
command = "制作我的四样东西"
mode = "image"
count = 0

[[shindan]]
id = "87845"
title = "少年！亮兵器吧！"
description = "在那剑与魔法的世界中~你将会成为什么？"
command = "剑与魔法"
mode = "image"
count = 0

[[shindan]]
id = "162924"
title = "测试你在动漫里面的样子"
description = "嗯哼"
command = "动漫中的样子"
mode = "image"
count = 0

[[shindan]]
id = "30052"
title = "不囉唆!!與你最搭配的動漫女角!!"
description = "哪個動漫女角與你最搭配呢~ 來玩玩吧~ 先提醒你-你大概會被惡搞..."
command = "动漫女角"
mode = "image"
count = 0

[[shindan]]
id = "318410"
title = "动漫角色测试"
description = "如果你今天穿越到动漫里了会是什么样子呢？是男还是女呢？会发生什么事？赶快来测试下吧！"
command = "动漫角色"
mode = "image"
count = 1

[[shindan]]
id = "1127800"
title = "动画片生成"
description = "随便用！每小时更新"
command = "动画片"
mode = "image"
count = 0

[[shindan]]
id = "1191352"
title = "来一句劲道的最终台词吧"
description = """
tips:较大的ooc可能性
脑壳抽筋的作品（）来一句劲道的台词吧！
95%是我自己编的，其余一小部分句子出自或致敬知名作品，相信你们能看得出来（）
如果冒犯到了的话，请留言告诉我，我会删掉的w
另外，如果想在别的地方借用我的自创台词的话可以加一下我qq说明，最好不要一声不吭就拿去用（我会很伤心的;w;）qq1781893325，也欢迎oc人扩列！
玩的开心！"""
command = "劲道台词"
mode = "image"
count = 1

[[shindan]]
id = "1136163"
title = "勇闯KPL"
description = """
请输入您的战队名
纯属娱乐 切勿当真
参考2022夏季赛选手榜+本人记忆"""
command = "勇闯KPL"
mode = "image"
count = 0

[[shindan]]
id = "830443"
title = "如果你是世间万物化成的"
description = """
如果人类是世间万物化形而成，你的组成结构会是怎样的呢？
一个奇怪的测试"""
command = "化成我"
mode = "image"
count = 0

[[shindan]]
id = "1136027"
title = "十个韵脚写段歌词"
description = "给你十个随机排列的韵脚，自定主题，你能否写出一段歌词？"
command = "十个韵脚"
mode = "image"
count = 0

[[shindan]]
id = "1139845"
title = "with十二月"
description = "你最近喜欢上了一个十二人女团，以下是你的属性"
command = "十二月女团"
mode = "image"
count = 1

[[shindan]]
id = "1051455"
title = "盘点那些色色的文梗【微重口预警】（单人-女性向）"
description = "盘点那些色色的文梗"
command = "单女色色文梗"
mode = "image"
count = 0

[[shindan]]
id = "360578"
title = "顔文字作るよ(  ﾟдﾟ )"
description = "今日の顔文字なんでしょう(  ﾟдﾟ )ﾊｯ! （日々顔文字追加中）"
command = "卖萌"
mode = "text"
count = 0

[[shindan]]
id = "1161422"
title = "卧龙里你的OC"
description = "光荣出品的三国诛死游戏"
command = "卧龙OC"
mode = "image"
count = 1

[[shindan]]
id = "1129108"
title = "蓋雷的歷史碎片"
description = """
我腦內映射世界的碎片
名字是密碼，而非人（可能會有些意義不明）
1（代表身份
2（代表仇人
3（代表歷史事件"""
command = "历史碎片"
mode = "image"
count = 0

[[shindan]]
id = "1118491"
title = "随机角色原画练习，脑洞向"
description = "每日三个词汇，适合给想走出舒适区的小伙伴们练习角色设计。会持续更新词库。"
command = "原画练习"
mode = "image"
count = 0

[[shindan]]
id = "1200442"
title = "随机角色原画练习，脑洞向进阶版"
description = "每日会随机一个角色职业。会持续更新词库。用于角色设计练习。"
command = "原画练习脑洞"
mode = "image"
count = 0

[[shindan]]
id = "1167768"
title = "原神，启动！"
description = "我超OP"
command = "原神启动"
mode = "image"
count = 0

[[shindan]]
id = "1150770"
title = "【原神／画像あり】全68種！2023年あなたと最も相性のいい原神キャラは！？"
description = """
原神に興味ある方、よかったらお試しください
2023年、あなたと相性のいい原神キャラは何でしょう？"""
command = "原神角色"
mode = "image"
count = 0

[[shindan]]
id = "528555"
title = "测试你会成为什么样的反派角色"
description = "(๑ŐдŐ)b"
command = "反派角色"
mode = "image"
count = 1

[[shindan]]
id = "1204678"
title = "当你变成游戏王卡会拥有什么效果"
description = """
当你变成游戏王卡会拥有什么效果
作者：牧良"""
command = "变成游戏王卡"
mode = "image"
count = 0

[[shindan]]
id = "638952"
title = "如果变成触手怪的话！（有污）"
description = "变成触手怪的话会怎么样！指定变量了（至少看起来像）技能和掉落物会根据种类，稀有度，等级变化！共有40种不同的触手+α（？）！装备取名好麻烦啊，所以低等级装备直接叫随机XX了。装备等级 普通-优秀-精良-史诗-传说-神器，共有神器12传说20.世界观和那个娘化穿越一样（ID:637918）不算那些超强能力称号的话那边最多单杀一只10W战斗力吧。用WOW参考的话上级低LV是精英怪高LV是5人本BOSS，稀有是10人BOSS，究极是RAID，高等级触手有时有惊喜~"
command = "变成触手怪"
mode = "image"
count = 0

[[shindan]]
id = "1154683"
title = "AIがあなたを可愛い二次元キャラクターに変身させます"
description = "あなたの名前から容姿をイメージして、AIが二次元用のプロフィールを考えてくれるよ。可愛く作れたらお持ち帰りください。結果の表示に少し時間がかかります。"
command = "变身可爱二次元"
mode = "image"
count = 1

[[shindan]]
id = "20263"
title = "你在古代的姓名字號"
description = "姓名採用百家姓,其餘一切請不要太認真...(笑)"
command = "古代姓名"
mode = "image"
count = 0

[[shindan]]
id = "1162250"
title = "古代家世生成器"
description = "给人物创造家庭背景的生成器（这里用的都是唐代官职）（可不换名字重新随机生成）"
command = "古代家世"
mode = "image"
count = 0

[[shindan]]
id = "35290"
title = "古代的身分"
description = "身為古代人的你會是什麼樣子？"
command = "古代的我"
mode = "image"
count = 0

[[shindan]]
id = "786329"
title = "古典笔名生成器"
description = "为你起一个明清才子风格的笔名"
command = "古典笔名"
mode = "image"
count = 0

[[shindan]]
id = "1133603"
title = "古风元素生成。"
description = "自用向随便搞。"
command = "古风元素"
mode = "image"
count = 0

[[shindan]]
id = "1133594"
title = "古风元素生成"
description = "自用"
command = "古风元素2"
mode = "image"
count = 0

[[shindan]]
id = "28206"
title = "你未來的另一半"
description = "你未來的另一半是什麽樣的"
command = "另一半"
mode = "image"
count = 0

[[shindan]]
id = "316802"
title = "你召唤出的英灵将是？"
description = "如果某一天你参加了圣杯战争你召唤出的生灵将会是谁呢～～恩 除去fate原作中出现的以外还加入了一些其他的英雄～"
command = "召唤英灵"
mode = "image"
count = 0

[[shindan]]
id = "316024"
title = "现在吃什么"
description = "午饭吃什么？晚饭吃什么？泡面？KFC？过来测试下吧。把测试的结果复制到G+发PO就可以得出结果啦。（注意：G+专供）"
command = "吃什么"
mode = "image"
count = 0

[[shindan]]
id = "1181412"
title = "今餐食咩(個人用)"
description = "呢個lunch又諗唔到食咩啦"
command = "吃什么2"
mode = "image"
count = 0

[[shindan]]
id = "1158015"
title = "外卖吃什么"
description = "自用外卖随机生成器"
command = "吃什么3"
mode = "image"
count = 0

[[shindan]]
id = "1180240"
title = "面對豆豆命運吧！"
description = """
你伸出手從袋子中抽出一顆柏蒂全口味豆，閉上雙眼準備迎接那或美味或噁心的味道。
豆子一入口，你便皺了皺眉，發現......"""
command = "吃豆豆"
mode = "image"
count = 0

[[shindan]]
id = "1211253"
title = "车一个后日谈的人偶吧！（固定版）"
description = "《自己动手丰衣足食地成为死灵师》"
command = "后日谈人偶"
mode = "image"
count = 0

[[shindan]]
id = "1132049"
title = "在这時 你听到了…"
description = """
【微猎奇】
聚集的祈愿化成新的光芒
无论悲痛或希望 你追寻着微弱的光……
【欢迎联系twi:@VittorioPuzo】"""
command = "听到了什么"
mode = "image"
count = 0

[[shindan]]
id = "1218122"
title = "今天听哪个消波块"
description = "来源于wiki"
command = "听哪个消波块"
mode = "image"
count = 0

[[shindan]]
id = "1115989"
title = "获得启示之刻"
description = "摸了，摆了，反正是架空，做着玩www"
command = "启示之刻"
mode = "image"
count = 0

[[shindan]]
id = "1166235"
title = "一句话命运"
description = "代餐用，oc或同人都可以。灵感来源是博尔赫斯。这是3.0，待修整。每个名字只对应一个结果。lof@浣熊公园"
command = "命运"
mode = "image"
count = 0

[[shindan]]
id = "1126144"
title = "如果在装满左右田和一的袋子中摸摸？"
description = "来看看你的和一口袋里都有什么吧～～是绝望的语擦人搞活，夹带了一些和女朋友的私心，保底有五星或者六星……但掉率还是很高的吧！"
command = "和一口袋"
mode = "image"
count = 0

[[shindan]]
id = "1142483"
title = "和三日月的本丸生活"
description = "哈哈哈哈，甚好甚好"
command = "和三日月的本丸生活"
mode = "image"
count = 0

[[shindan]]
id = "1116064"
title = "魔法部神奇动物在锅里社团的哈利波特魔法觉醒随机卡组生成器2"
description = "【哈利波特魔法觉醒】魔法部神奇动物在锅里社团的随机卡组生成器"
command = "哈利波特卡组"
mode = "image"
count = 0

[[shindan]]
id = "1088007"
title = "体验哈利波特魔法人生（2.0新版）"
description = """
全新版本，增加了更多毕业后经历，人生结局，在校事件。
作者微博：nickel拟科"""
command = "哈利波特魔法人生"
mode = "image"
count = 0

[[shindan]]
id = "1150916"
title = "哥们考试"
description = "选科默认全理了。分数为了不显得太离谱把下限调高了。"
command = "哥们考试"
mode = "image"
count = 4

[[shindan]]
id = "877212"
title = "哥谭的超幸运十连抽！"
description = "快来看看今天你抽到的是谁/情景/事件/台词吧！每天更新噢！"
command = "哥谭十连抽"
mode = "image"
count = 0

[[shindan]]
id = "1167846"
title = "噬神者系列里你的OC"
description = "可圈可点的共斗类狩猎游戏"
command = "噬神者"
mode = "image"
count = 0

[[shindan]]
id = "1167596"
title = "噬血代码里你的OC"
description = "和噬神者世界观相同的，操作【吸血鬼】战斗的魂系游戏"
command = "噬血代码"
mode = "image"
count = 0

[[shindan]]
id = "1148603"
title = "回忆生成器"
description = "想找寻你记忆深处的秘密吗"
command = "回忆"
mode = "image"
count = 0

[[shindan]]
id = "1136733"
title = "TA的回忆里有……"
description = "TA的记忆里有什么？（抽象表达）"
command = "回忆2"
mode = "image"
count = 0

[[shindan]]
id = "35169"
title = "你會困在夢的第幾層"
description = "又是怎樣的世界呢"
command = "困梦"
mode = "image"
count = 0

[[shindan]]
id = "1190757"
title = "那個人在做什麼呢？"
description = "讓我悄悄地看一眼ψ(｀∇´)ψ"
command = "在做什么"
mode = "image"
count = 0

[[shindan]]
id = "1120350"
title = "今天大家和誰在做什麼呢？"
description = "ψ(｀∇´)ψ"
command = "在做什么2"
mode = "image"
count = 0

[[shindan]]
id = "1120340"
title = "今天大家在做什麼呢？"
description = "共有833612個結果ψ(｀∇´)ψ"
command = "在做什么3"
mode = "image"
count = 0

[[shindan]]
id = "1204604"
title = "在艾欧泽亚的一生"
description = "如果那一天没有听到海德琳的呼唤，你的一生将会如何书写？"
command = "在艾欧泽亚的一生"
mode = "image"
count = 0

[[shindan]]
id = "1138848"
title = "绘画/写作场景灵感"
description = "可以用于画画和写作"
command = "场景灵感"
mode = "image"
count = 0

[[shindan]]
id = "843850"
title = "坠落"
description = "地狱风味，自娱自乐用"
command = "坠落"
mode = "image"
count = 0

[[shindan]]
id = "1153368"
title = "你是什么坦克战车娘"
description = """
战争雷霆x坦克世界x幻想时间
今天是什么姬车少女
最后更新2023.1.31
2023.1.19第一版
2023.1.29大改型号联动"""
command = "坦克战车娘"
mode = "image"
count = 0

[[shindan]]
id = "1160229"
title = "垃圾站里的幻想人设生成器/一日一刷新"
description = "毫无新意的乐色"
command = "垃圾站幻想人设"
mode = "image"
count = 0

[[shindan]]
id = "1161299"
title = "【自用】埃尔库利斯"
description = """
【【【自用】】】
仅共自己与友人试用，路人用了也看不懂

自设世界观埃尔库利斯

“你醒来后发现自己躺在一个陌生的港口，正在疑惑自己身处哪里的时候面前突然浮出了发着光的文字，而与此同时你的耳边响起了一个轻柔的女声——”"""
command = "埃尔库利斯"
mode = "image"
count = 0

[[shindan]]
id = "843854"
title = "墓志铭"
description = "地狱风味，自娱自乐用。9/29日更新。"
command = "墓志铭"
mode = "image"
count = 0

[[shindan]]
id = "1115622"
title = "你的墓碑前会有什么？"
description = "更大的可能是什么也没有。"
command = "墓碑前"
mode = "image"
count = 1

[[shindan]]
id = "1121834"
title = "你的夏天是？"
description = "欢迎回到盛夏！"
command = "夏天"
mode = "image"
count = 0

[[shindan]]
id = "828438"
title = "你的外在與內在的成分是？"
description = "每個人都有不同的外在和內在，來看看自己的組成吧"
command = "外在与内在"
mode = "image"
count = 0

[[shindan]]
id = "1204394"
title = "测测你在大征服者2的属性"
description = "输入名字测试你在大征服者2的属性和能力"
command = "大征服者2属性"
mode = "image"
count = 0

[[shindan]]
id = "1066594"
title = "测测你是大楚皇宫里面的谁？"
description = "穿越来到大楚的你，会是哪一个角色呢？"
command = "大楚皇宫"
mode = "image"
count = 0

[[shindan]]
id = "1194760"
title = "大陆酒店"
description = "你知道吗？有一家只在深夜十二点才能看见的酒吧！而且只有独自一个人的时候会出现！"
command = "大陆酒店"
mode = "image"
count = 1

[[shindan]]
id = "1215106"
title = "今日份pypy dance (按天变化)"
description = "vrchat"
command = "天命舞曲"
mode = "image"
count = 0

[[shindan]]
id = "896449"
title = "你轉生到異世界後得到的七個天賦技能"
description = "你轉生到異世界後得到的七個天賦技能"
command = "天赋技能"
mode = "image"
count = 0

[[shindan]]
id = "1136736"
title = "TA所失去的"
description = "TA失去了什么……"
command = "失去"
mode = "image"
count = 0

[[shindan]]
id = "790800"
title = "【自用世界观】奇妙的乱睡角色生成器"
description = "快乐乱睡"
command = "奇妙乱睡角色"
mode = "image"
count = 0

[[shindan]]
id = "1115924"
title = "oc/cp背景故事的奇妙意象"
description = "用于创作oc/角色/cp背景故事的一些奇妙意象或关键词"
command = "奇妙意象"
mode = "image"
count = 0

[[shindan]]
id = "790673"
title = "【自用世界观】奇妙的教会生成器"
description = "快乐教会"
command = "奇妙教会"
mode = "image"
count = 0

[[shindan]]
id = "790659"
title = "奇妙的角色生成器"
description = "快乐生成"
command = "奇妙角色"
mode = "image"
count = 0

[[shindan]]
id = "1142797"
title = "奇幻人设生成"
description = "个人向奇幻，性格和喜好可能会冲突"
command = "奇幻人设"
mode = "image"
count = 0

[[shindan]]
id = "1175394"
title = "宠物店里奇怪融化宠物的备注"
description = "是奇怪的融合宠物哒！（和亲友的oc随机骰子）"
command = "奇怪融化宠物"
mode = "image"
count = 0

[[shindan]]
id = "1218842"
title = "奈良物語2 【标准常驻池】[单次召唤]"
description = """
【标准常驻召唤】
召唤说明：使用奈良回忆×1600或者五连召唤卷×10可以进行一次召唤，召唤率相比于连续召唤大幅提高：
R———召唤率16% 
SR ———召唤率8%
SSR———召唤率1.6%
UR———召唤率0.6%

请在阅读以上事项后进行召唤，本游戏最终解释权由【爆钓计划】工作室拥有。"""
command = "奈良物语2"
mode = "image"
count = 0

[[shindan]]
id = "1218837"
title = "奈良物語2 【标准常驻池】[连续召唤]"
description = """
【标准常驻召唤】
召唤说明：使用奈良回忆×160或者五连召唤卷×1可以进行连续五次召唤，召唤率使用默认召唤率：
R———召唤率54% 
SR ———召唤率2.5%
SSR———召唤率0.5%
UR———召唤率0.2%

请在阅读以上事项后进行召唤，本游戏最终解释权由【爆钓计划】工作室拥有。"""
command = "奈良物语2连续召唤"
mode = "image"
count = 0

[[shindan]]
id = "870739"
title = "【R-18】你作为女奥特曼是怎么败北的"
description = "你作为女奥特曼是怎么败北的"
command = "女奥特曼败北"
mode = "image"
count = 0

[[shindan]]
id = "637294"
title = "人设生成器（女性限定）"
description = "批量产子 你值得拥有"
command = "女性人设"
mode = "image"
count = 0

[[shindan]]
id = "1135578"
title = "《数码兽抉择者》游戏企划终极抉择者人物卡（女）（固定版）（无VPN用）"
description = """
数码宝贝系列同人卡组构建类卡牌游戏（DBG）《数码兽抉择者》的人物卡，会不断更新更多更有意思的打法和流派。（这是固定版，用来测各位有缘的OC哟）
有兴趣了解的米娜桑欢迎+QQ：1658111549"""
command = "女数码兽抉择者"
mode = "image"
count = 0

[[shindan]]
id = "22157"
title = "女版的自己"
description = "女体化自身"
command = "女版自己"
mode = "image"
count = 0

[[shindan]]
id = "1126001"
title = "女神在抽卡（？）"
description = "女神在抽卡，是幸還是不幸？"
command = "女神在抽卡"
mode = "image"
count = 0

[[shindan]]
id = "1203714"
title = "女骑士生成器"
description = "成为幻想作品的女骑士行侠仗义，保护平民！"
command = "女骑士"
mode = "image"
count = 0

[[shindan]]
id = "1181376"
title = "今天娘化的你是什么样"
description = "魔法少女那边塞不下角色特征了，大概需要配合那边一起用"
command = "娘化的我"
mode = "image"
count = 0

[[shindan]]
id = "635902"
title = "娘化穿越到异世界ver4.2"
description = """
正经版完成
ID:637918
这里的技能我脑洞完说明后会把这里改成正经版格式|||||ID:638952触手诊断，触手诊断和正经版想通的，正经版不算那些超强的特殊能力和称号加成的话，正经版最多单杀一只10W战斗力吧。用WOW参考的话上级低LV是精英怪高LV是5人本BOSS，稀有是10人BOSS，究极是RAID"""
command = "娘化穿越"
mode = "image"
count = 0

[[shindan]]
id = "1092679"
title = "你娘化后在战锤40k宇宙生存"
description = "你娘化后在战锤40k宇宙生存！"
command = "娘化转生战锤"
mode = "image"
count = 0

[[shindan]]
id = "1210701"
title = "【娱乐圈】Y/N和他的邂逅"
description = "BG向 随机生成 很可能会出现怪答案但没关系（x）重试就好了"
command = "娱乐圈邂逅"
mode = "image"
count = 0

[[shindan]]
id = "760611"
title = "新孩子生成器"
description = "每日隨機角色生成 可以當作課題試著畫出來"
command = "孩子"
mode = "image"
count = 0

[[shindan]]
id = "1197151"
title = "欢迎来到宇宙中枢"
description = "【维和者，请递交个人信息】"
command = "宇宙中枢"
mode = "image"
count = 0

[[shindan]]
id = "1133873"
title = "测一测你是宋词中的什么意象"
description = "测一测你是宋词中的什么意象，用一组二字词来概括"
command = "宋词意象"
mode = "image"
count = 0

[[shindan]]
id = "1198169"
title = "创造完整的人设（基本版本）"
description = "完整的人设是对我而言，我不知道这个列表会不会让你觉得完整！总之就是这样，所以就这样吧（苍蝇搓手）.."
command = "完整人设"
mode = "image"
count = 0

[[shindan]]
id = "1160251"
title = "Your Pokémon Journey!!!"
description = "About Pokemon."
command = "宝可梦"
mode = "image"
count = 0

[[shindan]]
id = "833360"
title = "來飼養一只○○吧！"
description = "把自己或者角色的名字輸入☆-（ゝω・）v可能會有很多不完善的地方，抱歉【結果其實挺少的_(:з」∠)_"
command = "宠物培养"
mode = "image"
count = 0

[[shindan]]
id = "1204517"
title = "憧憬成为宠物萝莉"
description = """
不会真的有人想当宠物吧？
杂鱼杂鱼"""
command = "宠物萝莉"
mode = "image"
count = 0

[[shindan]]
id = "1134923"
title = "密教人物生成器"
description = "你会是密教模拟器世界里什么样的人？"
command = "密教人物"
mode = "image"
count = 0

[[shindan]]
id = "1135662"
title = "小劇場題材"
description = """
可以用来畫oc自設或者代餐cp之類
有搞笑因數和悲劇成分"""
command = "小剧场题材"
mode = "image"
count = 0

[[shindan]]
id = "828905"
title = "你是什么小动物？"
description = "测测你是什么小动物"
command = "小动物"
mode = "image"
count = 0

[[shindan]]
id = "1135885"
title = "小场景"
description = "oc跟那个ta的关系会以什么样的方式收场呢"
command = "小场景"
mode = "image"
count = 0

[[shindan]]
id = "1206040"
title = "小马宝莉马设oc生成"
description = """
职业和可爱标志可能冲突
魔法生物和可爱标志冲突
种族采用原作许多魔法生物"""
command = "小马宝莉OC"
mode = "image"
count = 0

[[shindan]]
id = "1206120"
title = "测测你与小马宝莉中的哪位角色是好朋友"
description = "包含原作主配角和路人"
command = "小马宝莉好朋友"
mode = "image"
count = 0

[[shindan]]
id = "1209956"
title = "少萝交友🔞"
description = "在寂寞的午夜和少萝宝宝一起❤️❤️"
command = "少萝交友"
mode = "image"
count = 0

[[shindan]]
id = "1163261"
title = "自用属性牌test"
description = "啊"
command = "属性牌"
mode = "image"
count = 0

[[shindan]]
id = "1124861"
title = "工地男人占卜「瑟瑟版」"
description = "满足你的一切幻想"
command = "工地男人"
mode = "image"
count = 0

[[shindan]]
id = "1220847"
title = "羽化状态___巨大娘生成器"
description = """
基于小说
PIXIV：《羽化状态》世界观制作的巨大娘生成器
随缘更新"""
command = "巨大娘"
mode = "image"
count = 18

[[shindan]]
id = "1197104"
title = "最能代表你的精神帕魯 (並沒有)"
description = """
我只是一段代碼，而你，我的朋友
診斷一結束之後你才是真正的帕魯
備註：這個診斷只是拿來搞朋友用的"""
command = "帕鲁"
mode = "image"
count = 0

[[shindan]]
id = "1135729"
title = "來一句帶感的臺詞"
description = """
可能ooc，可以用來代餐
*有玩梗元素*有摘录喜欢的文本
更新：删除了部分摘录文本
（有些文本参考了各种资料！侵删！制作者还不成熟侵权了请通知一下，这边会删除相关文本！聯繫方式→qq：3449014920）"""
command = "带感台词"
mode = "image"
count = 0

[[shindan]]
id = "28047"
title = "平行宇宙中的另一个你，此刻在做什么 ？"
description = "老艾克祝福大家"
command = "平行宇宙"
mode = "image"
count = 0

[[shindan]]
id = "1136770"
title = "来一个幸福的结局"
description = "来吧，少年哟！快来代餐！代你的oc，cp，自推吧！"
command = "幸福结局"
mode = "image"
count = 0

[[shindan]]
id = "1137229"
title = "今日幸運物"
description = "今日幸運物"
command = "幸运物"
mode = "image"
count = 0

[[shindan]]
id = "1150490"
title = "今日的幸運車次是......？"
description = "來看看今天與你最契合的車次是誰吧！"
command = "幸运车次"
mode = "image"
count = 0

[[shindan]]
id = "829668"
title = "你的幻境"
description = "幻境中你的身份是什麼樣的"
command = "幻境"
mode = "image"
count = 0

[[shindan]]
id = "1160234"
title = "是幻想和幻想啊！"
description = "随意的想象"
command = "幻想"
mode = "image"
count = 0

[[shindan]]
id = "1124532"
title = "幻想世界oc生成（不定时更新）"
description = """
偏向华丽重口的幻想世界主题
主女性
收录设圈常见元素
每次都不一样，不喜欢可以重抽哦"""
command = "幻想世界OC"
mode = "image"
count = 0

[[shindan]]
id = "1220025"
title = "幻想生物生成器"
description = "随机生成一个小动物绘画灵感"
command = "幻想生物"
mode = "image"
count = 0

[[shindan]]
id = "1208541"
title = "在颍川幼儿园兼职做老师"
description = "『三国』和颍川军师小朋友们度过了怎么样的一天？（张良乱入）"
command = "幼儿园兼职老师"
mode = "image"
count = 0

[[shindan]]
id = "1153256"
title = "让我康康你的姬尔"
description = """
杰哥：让我看看！
最后更新2023.1.19"""
command = "康康你的"
mode = "image"
count = 0

[[shindan]]
id = "1178059"
title = "又是一年开学季。。（剋泡女宝版）"
description = """
无聊产物 纯属娱乐

此为只有女宝版

预警* 有大量ooc，大概看个乐子就完事了。。
不要在意年龄方面 每个人都可能出现
是平行世界校园paro

并不认识所有人 基本上扒的百科

作者脑容量有限，可能没有你想随到人也可能会随到你不认识的whobe，踩雷了不要骂我，我4玻璃心。。。

如果想确认有没有某个人你可以联系我
没有的话想加我可以加"""
command = "开学季"
mode = "image"
count = 0

[[shindan]]
id = "1178000"
title = "又是一年开学季。。（捞版）"
description = """
无聊产物 纯属娱乐

lpl+少量lck

预警* 有大量ooc，大概看个乐子就完事了。。
不要在意年龄和语言方面 每个人都可能出现

是平行世界校园paro 圆他们的高中梦

主要是朋友整理的名单 脑容量有限 可能会有bug

可能没有你想随到人也可能会随到你不认识的，踩雷了不要骂我，我4玻璃心。。。

如果想确认有没有某个人你可以联系我
没有的话想加我可以加"""
command = "开学季2"
mode = "image"
count = 0

[[shindan]]
id = "1177996"
title = "又是一年开学季。。（农版）"
description = """
无聊产物 纯属娱乐

预警* 有大量ooc，大概看个乐子就完事了。。
不要在意年龄方面 每个人都可能出现

是平行世界校园paro 圆他们的高中梦

并不认识所有人 基本上扒的百科

作者脑容量有限，可能没有你想随到人也可能会随到你不认识的，踩雷了不要骂我，我4玻璃心。。。

如果想确认有没有某个人你可以联系我
没有的话想加我可以加"""
command = "开学季3"
mode = "image"
count = 0

[[shindan]]
id = "1177992"
title = "又是一年开学季。。（剋泡男宝版）"
description = """
无聊产物 纯属娱乐

此为只有男宝版

预警* 有大量ooc，大概看个乐子就完事了。。
不要在意年龄方面 每个人都可能出现
是平行世界校园paro

并不认识所有人 基本上扒的百科

作者脑容量有限，可能没有你想随到人也可能会随到你不认识的whobe，踩雷了不要骂我，我4玻璃心。。。

如果想确认有没有某个人你可以联系我
没有的话想加我可以加"""
command = "开学季4"
mode = "image"
count = 0

[[shindan]]
id = "1177980"
title = "又是一年开学季。。（内娱）"
description = """
无聊产物 纯属娱乐

内娱为男女混合版
但为了避免争议 cp只有南通女通 请放心食用。。。

预警* 有大量ooc，大概看个乐子就完事了。。
不要在意年龄方面 30以内的都在同学里
是平行世界校园paro

并不认识所有人 基本上扒的超话榜

作者脑容量有限，可能没有你想随到人也可能会随到你不认识的whobe，踩雷了不要骂我，我4玻璃心。。。

如果想确认有没有某个人你可以联系我
没有的话想加我可以加"""
command = "开学季5"
mode = "image"
count = 0

[[shindan]]
id = "1205029"
title = "异世界传送列车"
description = """
古老的预言终究还是应验了..
在我们所生活的世界，人类在与来自异空间的恶魔的战争中已经失去了绝大多数的领土。国家再也组织不起军队，人群中再也没有新的勇者。残存的人们龟缩在最后的要塞中，颤抖着，哭泣着，静待终幕降临。
一觉醒来，你却发现自己在一个不认识的世界中。神的声音在你脑中响起。你渐渐记起了自己的使命：驾驶你的列车，将拥有勇者资质的人送（创）来，为人类创造新的希望吧...
那么，你又会为世界带回怎样的希望呢？"""
command = "异世界传送列车"
mode = "image"
count = 1

[[shindan]]
id = "555143"
title = "如果你穿越/重生到異世界,你的屬性面板會是?"
description = "你可能會成為屬性逆天的胖子,也可能成為三圍豐滿的男生."
command = "异世界属性"
mode = "image"
count = 0

[[shindan]]
id = "738291"
title = "穿越轉生到異世界的狀態版面(升級豪華版)"
description = "穿越轉生到異世界的狀態版面(升級豪華版)"
command = "异世界状态版面"
mode = "image"
count = 0

[[shindan]]
id = "587874"
title = "異世界轉生—∩開始的種族∩——"
description = """
——到了你该重生的时刻了，那么开始测试种族吧
(武器随机制作中········）"""
command = "异世界转生"
mode = "image"
count = 0

[[shindan]]
id = "1191401"
title = "異世界轉生"
description = "好誒是異世界"
command = "异世界转生2"
mode = "image"
count = 0

[[shindan]]
id = "1187753"
title = "在異世界裡生活的話"
description = """
會是什麼樣的人呢？
//
2023/12/11：發佈，有空還會再更新。
2023/12/15：稍微更新了一些"""
command = "异世界转生3"
mode = "image"
count = 0

[[shindan]]
id = "1186635"
title = "被大卡车撞-重生之异世界故事"
description = "来点异世界故事试试，快来玩玩看"
command = "异世界转生4"
mode = "image"
count = 0

[[shindan]]
id = "1180180"
title = "看看异世界生活"
description = "随机人设"
command = "异世界转生5"
mode = "image"
count = 0

[[shindan]]
id = "1142688"
title = "异世界人生（未完成）"
description = "体验一把异世界的人生吧"
command = "异世界转生6"
mode = "image"
count = 0

[[shindan]]
id = "1189659"
title = "异能遊戲模擬器"
description = "你被捲入了一場异能遊戲，异能者們將相互廝殺，直到僅剩一人存活，獲勝者擁有實現一次願望的權利，來看看你在這場遊戲中的身份吧。"
command = "异能游戏"
mode = "image"
count = 0

[[shindan]]
id = "1140548"
title = "【原创世界观oc自用】来测测是怎样的异象猎人！"
description = """
个人【异象之示】系列世界观用
但欢迎来玩！！"""
command = "异象猎人"
mode = "image"
count = 0

[[shindan]]
id = "1167491"
title = "弈仙牌世界里你的人设"
description = "卡牌与修仙"
command = "弃仙牌OC"
mode = "image"
count = 0

[[shindan]]
id = "1130223"
title = "心之獸的構成原因"
description = """
心之獸的構成原因
（取最高的三項
希望 、心願（光
理想、 壓力（金
憤恨（火
憂傷（水
羨慕、忌妒（土
愛戀、色慾（木
恐懼、痛苦（暗
開心、快樂（風
靈光一閃的想法（雷"""
command = "心之兽"
mode = "image"
count = 0

[[shindan]]
id = "1130217"
title = "心之獸的素質"
description = """
力（攻擊、偵察）
智 （智力、反射）
速（速度、閃避）
精（為人處事、變通）
靈（潛力、靈性、創造力）"""
command = "心之兽素质"
mode = "image"
count = 0

[[shindan]]
id = "16164"
title = "测试你的潜能"
description = "那些你还没有被开发出来的必杀技"
command = "必杀技"
mode = "image"
count = 0

[[shindan]]
id = "1131429"
title = "车一个忍神角色吧（现代篇）"
description = "太过喜欢忍神所以就做了这个，只有现代流派"
command = "忍神角色"
mode = "image"
count = 0

[[shindan]]
id = "1205120"
title = "【換暱稱】快樂點心派對"
description = "你從桌上隨機抓了一種點心塞進嘴裡，嗯...真美味~"
command = "快乐点心派对"
mode = "image"
count = 0

[[shindan]]
id = "1134778"
title = "给你推荐一本快穿主受小说"
description = "耽美，快穿，主受，有强强也有笨蛋美人，也有切片……如果你最近文荒，恰巧喜欢看这种类型的小说的话，不妨点一下，一切随缘"
command = "快穿主受小说"
mode = "image"
count = 0

[[shindan]]
id = "1217702"
title = "是个怎样的人呢？"
description = """
是个怎样的人呢？
仅内在方面的生成
外表生成：ID:1215134"""
command = "怎样的人"
mode = "image"
count = 0

[[shindan]]
id = "1162222"
title = "你的思绪是什么"
description = "一些意识流的产物>< 没有任何真实性！纯娱乐！"
command = "思绪"
mode = "image"
count = 0

[[shindan]]
id = "1148670"
title = "岔皮大爆炸"
description = """
自用自用自用！自己和朋友用！
个人与朋友的xp，自用，r18、g向、重口。"""
command = "性癖"
mode = "image"
count = 0

[[shindan]]
id = "1137909"
title = "怪物猎人世界里你的OC"
description = "卡普空出品的大热门共斗类狩猎游戏。"
command = "怪物猎人世界"
mode = "image"
count = 0

[[shindan]]
id = "1142562"
title = "怪物猎人崛起里你的OC"
description = "卡普空出品的大热门共斗类狩猎游戏——第二弹。"
command = "怪物猎人崛起"
mode = "image"
count = 0

[[shindan]]
id = "1090825"
title = "恋爱观"
description = "#結婚"
command = "恋爱观"
mode = "image"
count = 0

[[shindan]]
id = "1158309"
title = "惡作劇時間!是躲過一劫還是被迫中獎?"
description = "惡作劇時間!在這個滿是小雪球和屎炸彈的季節，你是躲過一劫還是被迫中獎?"
command = "恶作剧"
mode = "image"
count = 0

[[shindan]]
id = "1140859"
title = "你是悲剧的主人公"
description = """
各种各样的悲剧，适用于为oc提供灵感，命题作文/绘图等。
结果日替，不定期更新。"""
command = "悲剧主人公"
mode = "image"
count = 0

[[shindan]]
id = "868441"
title = "SPANKING自我惩罚生成器"
description = "输入你的名字，就可以根据你今天的犯错情况来生成一个属于你的DIY惩罚要求，请老老实实做哦！"
command = "惩罚"
mode = "image"
count = 0

[[shindan]]
id = "24963"
title = "不要猶豫！成為一本書吧！(認真ver.0.1)"
description = "你會是怎麼樣的一本書呢？"
command = "成为书"
mode = "image"
count = 0

[[shindan]]
id = "1146263"
title = "测测你喜欢的人是谁☆〜（ゝ。∂）"
description = "完全准确 千真万确"
command = "我喜欢的人"
mode = "image"
count = 0

[[shindan]]
id = "761425"
title = "你是什麼做成的"
description = "讓我來分析你是由哪些東西做成的吧（以鵝媽媽童謠為基礎）"
command = "我是什么做成的"
mode = "image"
count = 0

[[shindan]]
id = "829369"
title = "你是什麼做成的2.0"
description = """
較原版迷幻的版本
原測驗：
ID:761425"""
command = "我是什么做成的2"
mode = "image"
count = 0

[[shindan]]
id = "1083312"
title = "穿越到二次世界大战的你会是什么样将军呢？"
description = "沉迷钢铁雄心，巨舰大炮主义受害者的我做了这个测试，怎么说呢……各位玩的开心就好。"
command = "我是什么将军"
mode = "image"
count = 0

[[shindan]]
id = "29060"
title = "你是甚麼控"
description = "你到底是控甚麼呢?還有你的糟糕程度是?"
command = "我是什么控"
mode = "image"
count = 0

[[shindan]]
id = "27587"
title = "我最喜歡...樣的人了!! 0////x////0"
description = "這個測驗很私心.. 大多數都是我愛的型~ (羞掩面"
command = "我最喜欢的人"
mode = "image"
count = 0

[[shindan]]
id = "21565"
title = "測試你有多S"
description = "真的就只是單純測試你有多S (18禁有) (?)"
command = "我有多S"
mode = "image"
count = 0

[[shindan]]
id = "827537"
title = "测试你爱的人是谁？"
description = "输入你的名字"
command = "我爱谁"
mode = "image"
count = 0

[[shindan]]
id = "1143582"
title = "你是由什么构成的？【古代篇】"
description = "大梦一场。"
command = "我由什么构成"
mode = "image"
count = 0

[[shindan]]
id = "1136513"
title = "你是由什么组成的？"
description = "你是由什么组成的？"
command = "我由什么组成"
mode = "image"
count = 0

[[shindan]]
id = "845842"
title = "用三句话概括CP"
description = "偏虐向。"
command = "我的一生"
mode = "image"
count = 0

[[shindan]]
id = "1151548"
title = " 在……你……的故事"
description = """
即兴创作，不定期更新wwwww
一些有感而发……"""
command = "我的故事"
mode = "image"
count = 0

[[shindan]]
id = "829511"
title = "你的爱是怎样的？"
description = "心中美好与岁月遗憾"
command = "我的爱"
mode = "image"
count = 0

[[shindan]]
id = "1070601"
title = "粉丝构成测试"
description = "测试您的粉丝构成"
command = "我的粉丝"
mode = "image"
count = 0

[[shindan]]
id = "1211825"
title = "战争大陆随机旅游挑战！"
description = """
//一切设定基于个人原创世界观「奥尔特手记」

——
“欢迎来到战争大陆——「银河车站」！
资料显示您是首次登陆这里的旅客，那么……

要不要来尝试一下我们新推出的「随机旅游规划」服务呢？”"""
command = "战争大陆"
mode = "image"
count = 0

[[shindan]]
id = "1121766"
title = "【戰雙】今天的你會遇到誰?"
description = "罪孽深重的灰鴉指揮官啊，今天又去哪裡拈花惹草了?"
command = "战双"
mode = "image"
count = 0

[[shindan]]
id = "17507"
title = "战斗力"
description = "综合战斗能力判断"
command = "战斗力"
mode = "image"
count = 0

[[shindan]]
id = "1198952"
title = "寻求战斗的人"
description = "给想创建战斗角色的你。"
command = "战斗角色"
mode = "image"
count = 0

[[shindan]]
id = "832039"
title = "當你變成了戰術人形☆"
description = "背景是少女前線w之前是在別的網站上做的，搬運過來了_(:з」∠)_"
command = "战术人形"
mode = "image"
count = 0

[[shindan]]
id = "21431"
title = "適合待在您身邊的執事與女僕是？"
description = "可以召喚出各種類型的執事與女僕。"
command = "执事与女仆"
mode = "image"
count = 1

[[shindan]]
id = "1126910"
title = "假設你穿越到執劍之刻裡的設定（會更新）"
description = """
作者第一次做的，如有問題請在留言區提供建議。
如果有甚麼想要玩玩看的也可以許願呦☆
設定來自遊戲《執劍之刻》"""
command = "执剑之刻"
mode = "image"
count = 0

[[shindan]]
id = "995775"
title = "奇异少年抓捕装置1"
description = "是我流醒脾集合的少年扭蛋，合集2在个人首页里，含强迫r18元素，有兴趣可以看看。"
command = "抓捕少年1"
mode = "image"
count = 0

[[shindan]]
id = "1051544"
title = "奇异少年诱捕装置2"
description = "接1 ，1是少年的大方向 ，2是细节补充。"
command = "抓捕少年2"
mode = "image"
count = 0

[[shindan]]
id = "995787"
title = "奇异少年抓捕装置3"
description = "接1 ，1是少年的大方向 ，2是细节补充，3是r18设定补充。"
command = "抓捕少年3"
mode = "image"
count = 0

[[shindan]]
id = "1147683"
title = "投胎到中国！"
description = "看看自己投胎到哪个省吧！"
command = "投胎中国"
mode = "image"
count = 0

[[shindan]]
id = "1150686"
title = "你的P3老婆"
description = "抽取在《女神异闻录3》中，你的老婆"
command = "抽P3老婆"
mode = "image"
count = 0

[[shindan]]
id = "1150609"
title = "抽取P4老婆"
description = "抽取P4老婆"
command = "抽P4老婆"
mode = "image"
count = 0

[[shindan]]
id = "1150594"
title = "抽P5老婆"
description = "抽取P5老婆"
command = "抽P5老婆"
mode = "image"
count = 0

[[shindan]]
id = "1000507"
title = "あなたに似ているジャンプ主人公診断！"
description = """
あなたに似ているジャンプ主人公診断です！
1日1回、あなたに似ているジャンプの主人公を
あなたの名前から読み取って、画像で診断します！好評でしたらジャンプ主人公追加予定！"""
command = "抽丈夫"
mode = "image"
count = 0

[[shindan]]
id = "1142582"
title = "あなたが行ったらハマるホスト"
description = "ホスト大〜i好き！！！！！！"
command = "抽主人"
mode = "image"
count = 0

[[shindan]]
id = "1099408"
title = "Atom-Element"
description = "就是元素"
command = "抽元素"
mode = "image"
count = 0

[[shindan]]
id = "1150017"
title = "抽卡模拟器（待修改）"
description = """
输入抽卡角色进入他的卡池
R—65%
SR—20%
SSR—10%
SP—5%

糖/HE⭕️
刀/BE⚠️
车🔞
片段/NE"""
command = "抽卡模拟"
mode = "image"
count = 0

[[shindan]]
id = "1163454"
title = "测试你是原神中的谁"
description = "输入名称测试你是原神中的角色"
command = "抽原神"
mode = "image"
count = 0

[[shindan]]
id = "1163576"
title = "测试你是原神形象简易版"
description = "这是简易版，完整版在这ID:1163454完整版施工中"
command = "抽原神简易"
mode = "image"
count = 0

[[shindan]]
id = "1044378"
title = "地名抽抽"
description = """
來~~看看你跟台灣的各個地名有多少緣分吧？抽籤內容會慢慢更新~~（最新更新 2021/3/5）
目前星級數：★1：10、★2：8、★3：7、★4：5、★5：4

圖片皆為作者自繪。"""
command = "抽台湾地名"
mode = "image"
count = 0

[[shindan]]
id = "1150872"
title = "Persona5塔罗"
description = "随机抽取一张塔罗牌"
command = "抽塔罗"
mode = "image"
count = 4

[[shindan]]
id = "1199388"
title = "给oc或者cp抽一张塔罗牌吧"
description = "非专业占卜人事，只是纯粹出于兴趣制作的测试，切勿当真随便玩玩吧。对于牌的含义可自行google、百度（？）"
command = "抽塔罗2"
mode = "image"
count = 0

[[shindan]]
id = "1150610"
title = "抽取女神异闻录老婆"
description = "抽取P3、P4、P5的老婆们"
command = "抽女神异闻录老婆"
mode = "image"
count = 0

[[shindan]]
id = "1075116"
title = "あなたの二次元での嫁ヒロイン"
description = "この診断では、もしもあなたが二次元の住人だったら結婚するであろう嫁(ヒロイン)を診断します！"
command = "抽妻子"
mode = "image"
count = 0

[[shindan]]
id = "1221873"
title = "抽にじさんじ彩虹社主播"
description = "随机抽一个にじさんじ彩虹社（本社）的主播"
command = "抽彩虹社主播"
mode = "image"
count = 0

[[shindan]]
id = "1135573"
title = "《数码兽抉择者》游戏企划终极抉择者人物卡（女）（变动版）"
description = """
数码宝贝系列同人卡组构建类卡牌游戏（DBG）《数码兽抉择者》的人物卡，会不断更新更多更有意思的打法和流派。（这是变动版，用来任意车喜欢的OC）
有兴趣了解的米娜桑欢迎+QQ：1658111549"""
command = "抽数码兽抉择者"
mode = "image"
count = 0

[[shindan]]
id = "541545"
title = "《永恒之轮》！看看您驾驶哪一部星武装机！"
description = "河图发见！星武横空！"
command = "抽星武"
mode = "image"
count = 0

[[shindan]]
id = "1110781"
title = "你的特殊能力是什么？"
description = """
异能之类的东西，可以作为人设创作参考。
随便做的随便玩玩"""
command = "抽特殊能力"
mode = "image"
count = 0

[[shindan]]
id = "1208686"
title = "儿童节女巫糖果抽抽乐！"
description = "呀，是谁在这里放了一个糖果箱？"
command = "抽糖果"
mode = "image"
count = 0

[[shindan]]
id = "876174"
title = "あなたっぽい主人公キャラ"
description = "皆さんジャンプは好きですか？僕は、好きです！と言ってもジャンプはあまり読まない方なのですが、ジャンプのアニメ作品は大好きな作品が多いので、昨日今日とで画像を懸命に探して、この診断を作ってみました！ジャンプ好きの方なら誰もが知っているであろう昭和〜平成のジャンプ主人公が勢ぞろい！あなたっぽいジャンプ主人公とは、一体誰なのでしょうか！？今すぐチェックしよう！追記(ご好評につき、皆様のおかげで10万人突破しました！2日目からはジャンプキャラ以外の主人公も出現するようになりました！)"
command = "抽老公"
mode = "image"
count = 0

[[shindan]]
id = "1150687"
title = "抽老婆"
description = """
抽取二次元老婆
随缘更新图库，前提是有人玩（

PS:两三年前做的东西怎么突然火了wc,关注作者B站 UID367209850 谢谢喵"""
command = "抽老婆"
mode = "image"
count = 0

[[shindan]]
id = "1150888"
title = "能量属性"
description = """
(咱第一次玩我都不知道能出来什么东西)
好！修完善了大家可以随便玩了！是测定oc能量属性分布和总能量值的一个随机数roll（？） 为了方便读取数据特地帮大家除了一下占满分的百分之多少x"""
command = "抽能量属性"
mode = "image"
count = 1

[[shindan]]
id = "400813"
title = "【艦これ】あなたの嫁になる艦娘は？"
description = "現在、出ている艦娘の中から1体だけ選択されます。(改、甲、航は無し)　New!→(9/3)夏イベで実装された新艦を追加しました。"
command = "抽舰娘"
mode = "image"
count = 0

[[shindan]]
id = "361845"
title = "マイ・ガンダム診断"
description = """
量産型MSバージョン作りました→ID:592376
1stから最新作までガンプラ機体含め入れています 7/12バルバトスルプスとスクランブル追加
ついったーさんに紹介されました→https://twitter.com/tsuitta_san/status/..."""
command = "抽高达"
mode = "image"
count = 0

[[shindan]]
id = "1214015"
title = "拉丁语系外文名字生成器"
description = """
实际上是音译名字常用字字库的随意堆叠
完全没有逻辑
或许会偏男性向一些
做参考用
（另：如果有谁随机出了“巴巴托斯”或者“斯莱特林”这种请务必QQ联系我：3689735515！因为这真的很好笑！我也要笑笑~"""
command = "拉丁语名字"
mode = "image"
count = 0

[[shindan]]
id = "1137539"
title = "你属于拉尼卡公会城的哪个公会呢？"
description = "“拉尼卡公会城”是著名TCG《万智牌》里的一个十分重要的时空，它是一个由十会盟所支配的超大城市。那么，来测测你属于那个公会吧"
command = "拉尼卡公会城"
mode = "image"
count = 0

[[shindan]]
id = "35205"
title = "你的擬人動物代表"
description = "你會是怎麼樣的動物呢?"
command = "拟人动物"
mode = "image"
count = 0

[[shindan]]
id = "870843"
title = "【R-18】被拷问的女奥特曼是……"
description = "被拷问的女奥特曼是……"
command = "拷问女奥特曼"
mode = "image"
count = 0

[[shindan]]
id = "1209263"
title = "群友今天去哪挂机！1.0"
description = "今天去哪挂机呢（仅鸟区主城）"
command = "挂机地"
mode = "image"
count = 0

[[shindan]]
id = "1133869"
title = "你在指环王里是什么元素"
description = "测一测你是指环王世界中的什么元素"
command = "指环王元素"
mode = "image"
count = 0

[[shindan]]
id = "1116035"
title = "你将会在路边捡到什么"
description = "使用三个一览表可生成较复杂文本的占卜。"
command = "捡到什么"
mode = "image"
count = 0

[[shindan]]
id = "1177918"
title = "pathfinder里你的OC"
description = "对没错，就是那个全球有名的跑团啦"
command = "探索者OC"
mode = "image"
count = 0

[[shindan]]
id = "1169935"
title = "揍人学院学生证"
description = "你坐牢前读的最后一个学校"
command = "揍人学院学生证"
mode = "image"
count = 0

[[shindan]]
id = "1150508"
title = "當你進入提奧耶工作......"
description = """
原創世界觀，會慢慢更新事件~
與車次少女們的工作日常會是如何呢？"""
command = "提奥耶工作"
mode = "image"
count = 0

[[shindan]]
id = "828977"
title = "你是什么故事的主角呢"
description = "给想开企划，想生孩子但是还没有脑洞的各位"
command = "故事主角"
mode = "image"
count = 0

[[shindan]]
id = "1121649"
title = "你故事的一個片段(未完成)"
description = """
某時某刻、某個人正在……
一個場景 一個身份 一種情況，某個故事的一個片段
*30%編輯階段 未完成 僅供測試"""
command = "故事片段"
mode = "image"
count = 0

[[shindan]]
id = "1155635"
title = "随机故事背景设定"
description = """
*适合OC人
1. 什么时候
2. 在哪里
3. 做了什么
4. 结局"""
command = "故事背景"
mode = "image"
count = 0

[[shindan]]
id = "1121982"
title = "正义t爆！！"
description = "总之就是浅做一点那个什么的要素涩涩实验"
command = "敏感度"
mode = "image"
count = 0

[[shindan]]
id = "1204649"
title = "散触老黄历 Ver1.2"
description = "据说这是怪物猎人间代代传承的老黄历，出发触S前确定不看看吗？"
command = "散触老黄历"
mode = "image"
count = 0

[[shindan]]
id = "1135572"
title = "《数码兽抉择者》游戏企划终极抉择者人物卡（女）（固定版）"
description = """
数码宝贝系列同人卡组构建类卡牌游戏（DBG）《数码兽抉择者》的人物卡，会不断更新更多更有意思的打法和流派。（这是固定版，用来测各位有缘的OC哟）
有兴趣了解的米娜桑欢迎+QQ：1658111549"""
command = "数码兽抉择者"
mode = "image"
count = 0

[[shindan]]
id = "1206263"
title = "在数码世界的搭档数码兽是谁"
description = "在数码世界的搭档数码兽"
command = "数码搭档兽"
mode = "image"
count = 0

[[shindan]]
id = "1205065"
title = "斗罗大陆 --- 来测试你的 ‘武魂’ 吧 ---- 已制作完成"
description = "来自中国的修真oc 测试器 欢迎大家测试自己的 ‘斗罗武魂’"
command = "斗罗武魂"
mode = "image"
count = 0

[[shindan]]
id = "1133418"
title = "SAMPLE : CHART函数 TABLE图表"
description = "是使用图表函数 =CHART() 生成表格的占卜。"
command = "新班级座位表"
mode = "image"
count = 0

[[shindan]]
id = "1126136"
title = "來自施法應用圖鑑的奇妙用法"
description = """
（不建議用本名
（怪怪的很正常
（這是一時興起，不完整"""
command = "施法应用图鉴"
mode = "image"
count = 0

[[shindan]]
id = "1169069"
title = "无常法资质测定"
description = """
《苍穹都市的超能力者》中专门应对龙灾的屠龙术——无常法，分为寂空灵梵荒祸奇七相，灵照•明晰•通神•显现•创界五境。
本测验中十字星（✧/✦）代表（正向/负向）契合度，□代表成长性。
无常法资质值由无常法使自身的成长性与契合度相加，再根据契合度正负评判（例如5□+3✦=-8分）。成长性意味着自身的天赋或可发展性。正向契合度则意味着稳定，高效，高可控。而负向契合度意味着波动，超速，低可控。一般而言资质被判定为负，就绝不能修行该心相的无常法，但本次测试只判定无常法使的资质高低与潜力，不对正负倾向具体评价。"""
command = "无常法资质"
mode = "image"
count = 1

[[shindan]]
id = "1165318"
title = "无意义拼贴"
description = "多试试，说不定就有有意义的句子了"
command = "无意义拼贴"
mode = "image"
count = 0

[[shindan]]
id = "1165322"
title = "无意义拼贴2"
description = "多试试，说不定就会有有意义的语句呢"
command = "无意义拼贴2"
mode = "image"
count = 0

[[shindan]]
id = "1165317"
title = "无意义随机拼贴"
description = "多试试，万一就有有意义的句子呢"
command = "无意义拼贴3"
mode = "image"
count = 0

[[shindan]]
id = "454982"
title = "沒梗時的好幫手"
description = """
輸入CP名等等,文或圖都可以,從甜到虐到搞笑,總之沒梗時玩玩無妨。
狀態/"""
command = "无梗助手"
mode = "image"
count = 0

[[shindan]]
id = "1092787"
title = "角色被葬入无畏机甲"
description = "角色被放入了无畏机甲后的故事。"
command = "无畏机甲"
mode = "image"
count = 0

[[shindan]]
id = "1150177"
title = "请勿在无限流游戏里找男朋友！"
description = "输入你的姓名进入游戏"
command = "无限流男友"
mode = "image"
count = 0

[[shindan]]
id = "1161245"
title = "日落賭場"
description = "賭輸失去的不一定是錢"
command = "日落赌场"
mode = "image"
count = 0

[[shindan]]
id = "1135818"
title = "每日時裝製造機"
description = "給自己設計的動力，之後再慢慢增加東西(等我想到的話)"
command = "时装"
mode = "image"
count = 0

[[shindan]]
id = "27865"
title = "一生中某件事占時間"
description = "一生中哪件事占時間"
command = "时间占比"
mode = "image"
count = 0

[[shindan]]
id = "905435"
title = "明日方舟干员生成"
description = "明日方舟干员生成器 无每日变更"
command = "明日方舟干员"
mode = "image"
count = 0

[[shindan]]
id = "1186773"
title = "明日方舟干员制成|随机干员生成"
description = "昨夜圆车干员职业生成，附带脚臭指数"
command = "明日方舟干员2"
mode = "image"
count = 0

[[shindan]]
id = "1156225"
title = "明日方舟干员生成器"
description = "大家图一乐就好，别太认真(`･ω･´)"
command = "明日方舟干员3"
mode = "image"
count = 0

[[shindan]]
id = "1124214"
title = "明日方舟干员生成 职业基础档案客观履历"
description = "基础资料信物专精密录"
command = "明日方舟干员4"
mode = "image"
count = 0

[[shindan]]
id = "897114"
title = "明日方舟角色生成器"
description = """
生成明日方舟角色
https://ak.hypergryph.com/index"""
command = "明日方舟角色"
mode = "image"
count = 0

[[shindan]]
id = "959146"
title = "明日方舟风格干员生成器（ALPHA）"
description = """
明日方舟干员生成器，结果为日更型。基本上按照干员图鉴的格式录入，略有删减，总结果数已经突破天际。可能会略有冲突，请多担待。由于字数限制，没有3星以下。未实装（甚至不可能实装）的组合注意。
更新大概不存在。艹生不可避。"""
command = "明日方舟风格干员"
mode = "image"
count = 0

[[shindan]]
id = "1206193"
title = "在崩坏：星穹铁道，你会是什么样的人？"
description = "践行命途可不是游戏的7命途哦"
command = "星穹铁道OC"
mode = "image"
count = 0

[[shindan]]
id = "1092608"
title = "角色成为星际战士的战斗力"
description = "该角色成为忠诚罐头的战斗力"
command = "星际战士"
mode = "image"
count = 0

[[shindan]]
id = "1155175"
title = "清河春日幻境之旅"
description = """
時光輪轉，轉眼間，盎然綠意逐漸覆蓋了清河。城主和武將們都有了出行賞春的意願。但是武將衆多，一起出行難免興師動衆。
那麽，誰是最適合城主大人這趟春日行的最佳伴侶呢？快測試一下吧！"""
command = "春日幻境"
mode = "image"
count = 0

[[shindan]]
id = "1159796"
title = "性格&喜欢&讨厌"
description = """
适合OC人
随便玩玩
随缘更新"""
command = "是什么人"
mode = "image"
count = 0

[[shindan]]
id = "1125771"
title = "占卜你的智力度"
description = "案例在一览表中输入 0～100 的数值，可以制作出用于“占卜出你的智力度”的小占卜"
command = "智力度"
mode = "image"
count = 0

[[shindan]]
id = "1176458"
title = "Superior oc generator（更酷的角色卡生成器）"
description = """
基于“很酷的角色卡生成器”时隔多年的重制版，同样是原创世界观的设定，无论阁下因为什么原因来玩这个，都希望这些设定能帮上你的忙！至少玩的开心！
对原创世界观有兴趣的话欢迎加我的qq1781893325！"""
command = "更酷的角色卡"
mode = "image"
count = 0

[[shindan]]
id = "1117882"
title = "你曾经是什么？"
description = "可以测自己，oc或者喜欢的角色！"
command = "曾经是什么"
mode = "image"
count = 0

[[shindan]]
id = "522194"
title = "看看你的替身是什么样的"
description = "输入你的名字，看看你的替身的属性吧"
command = "替身属性"
mode = "image"
count = 0

[[shindan]]
id = "905484"
title = "JOJO替身能力"
description = "【星尘斗士风？】替身名+图表+一点说明"
command = "替身能力"
mode = "image"
count = 0

[[shindan]]
id = "1144712"
title = "最终幻想14里你的OC"
description = "史克威尔艾尼斯出品的全球热门MMORPG游戏"
command = "最终幻想14"
mode = "image"
count = 0

[[shindan]]
id = "1138042"
title = "【梅吉库塔】今晚吃到的月饼是什么口味的？"
description = "都只有一个晚上的效果哦~"
command = "月饼"
mode = "image"
count = 0

[[shindan]]
id = "1160390"
title = ">服装主题< 角色生成器"
description = "服装设计主题的角色生成器，推荐用来提供角色设计的灵感！每日更换"
command = "服装主题角色生成"
mode = "image"
count = 0

[[shindan]]
id = "1131250"
title = "服裝風格靈感"
description = "自用"
command = "服装风格灵感"
mode = "image"
count = 0

[[shindan]]
id = "1177098"
title = "未来事物灵感生成器：The Thing from the Future Card Game"
description = """
搬运和汉化situationlab制作的FutureThing灵感卡片，用的时候觉得做成生成器会很方便。这个生成器可以在设计世界观，发散思维等等时使用，帮助就未来不同领域的具体事物进行发散思考，并提供了相应的方向。
我还不太会用shindanmaker，请见谅
源地址: https://situationlab.org/futurething-print-a..."""
command = "未来事物"
mode = "image"
count = 0

[[shindan]]
id = "1141174"
title = "准备好与你未来的恋人相遇了吗？"
description = "（世界观:奇幻世界，科技水平依地区各有不同）"
command = "未来恋人"
mode = "image"
count = 0

[[shindan]]
id = "1146780"
title = "《希诺尔奇的诅咒》之末日祭典"
description = """
【末日祭典】 是天使们的祭典， TA们虽然不喜欢热闹，不喜欢与众不同。
但是这周，是天使们释放压力的时候。 
去年我去度假泡温泉的时候，不小心让 【所罗门】 跑了。
我不指望你们能毁掉TA，但是祝你们好运。"""
command = "末日祭典"
mode = "image"
count = 0

[[shindan]]
id = "443593"
title = "测试你的本命武器"
description = """
从2014年5月6日开始，你将获得更多的专属武器
取自http://tieba.baidu.com/p/2006264091"""
command = "本命武器"
mode = "image"
count = 0

[[shindan]]
id = "1117121"
title = "网络式机叶抽奖"
description = "爬老师倾情巨献"
command = "机叶十连"
mode = "image"
count = 1

[[shindan]]
id = "1123765"
title = "杀人的理由"
description = "可能测出好笑的东西（可能"
command = "杀人理由"
mode = "image"
count = 0

[[shindan]]
id = "1141522"
title = "你是一名杀手"
description = "杀手的人生不需要解释"
command = "杀手"
mode = "image"
count = 0

[[shindan]]
id = "829594"
title = "会杀死你的东西"
description = "是什么呢"
command = "杀死我的东西"
mode = "image"
count = 0

[[shindan]]
id = "820217"
title = "♣我有梗，你有笔吗♣"
description = """
结果格式：
♣CP属性♣
♣CP关系♣
♣关键词♣
♣预设剧情♣
♣插播对白♣
*内容比较广泛，随机出的结果可能很奇怪，咳咳。"""
command = "来梗"
mode = "image"
count = 0

[[shindan]]
id = "1133069"
title = "今天要來點什麼梗？"
description = """
沒有靈感的時候歡迎來抽抽看，一抽三梗，每日變更，有糖有刀
輸入名字或CP名都行，如有雷同，純屬巧合"""
command = "来梗2"
mode = "image"
count = 0

[[shindan]]
id = "1118288"
title = "你來自何方？"
description = """
我和我的朋友们，来自何方呢？
【欢迎联系twi:@VittorioPuzo】
【更新说明:上面那个号被封了 也没人联系我 我就不贴新的了】"""
command = "来自何方"
mode = "image"
count = 0

[[shindan]]
id = "1135903"
title = "测测你来自何方，下一个目的地又会是哪里"
description = "肝不起了，随便做着玩玩，可以oc 代餐欢迎转发"
command = "来自何方2"
mode = "image"
count = 0

[[shindan]]
id = "1062457"
title = "如果你是架空世界中的角色"
description = "输入角色名，测试你的属性"
command = "架空角色"
mode = "image"
count = 0

[[shindan]]
id = "1090381"
title = "请查收你的魅☆魔档案"
description = """
含r18和g向，已经放飞无底线了！注意闪避噢☆
私心有一点彩蛋～很不明显但是懂的人自然都懂..."""
command = "查收魅魔"
mode = "image"
count = 0

[[shindan]]
id = "637314"
title = "來自葛籣希爾大陸夢境的旅行者"
description = """
歡迎你，來到夢境之間的旅行者們
16/6/27更新：
性格描述修正&追加"""
command = "梦境旅行者"
mode = "image"
count = 0

[[shindan]]
id = "1210699"
title = "【个人使用】到处搜刮来的梗喵喵喵"
description = """
梦女向BG向 注意避雷
什么都有 大部分是甜 看之前先寄存脑子 食用愉快"""
command = "梦女BG梗"
mode = "image"
count = 0

[[shindan]]
id = "1136794"
title = "TA的梦想"
description = """
抽象式表达
梦想可以是想去的地方，想做的事，想见的人……等等"""
command = "梦想"
mode = "image"
count = 0

[[shindan]]
id = "759939"
title = "✦夢病塔✧"
description = """
『距離上次清醒已經又睡了多久……還是說，現在也是在做夢呢？』
你從催眠療程醒過來，卻發現自己躺在陌生的醫療機構裏；難以理解的實驗、隔離，獲得不可思議能力的代價是在永夜的醫療塔中，所有的人都深陷睡夢不可自拔……"""
command = "梦病塔"
mode = "image"
count = 0

[[shindan]]
id = "1142219"
title = "你的梦里会出现些什么？"
description = "因为是梦所以不存在逻辑，占卜结果每日变化"
command = "梦里会看见什么"
mode = "image"
count = 0

[[shindan]]
id = "1156904"
title = "來畫棉花娃稿吧"
description = "作者詞彙量有限，使用後繪製出來的娃稿跟別人撞到相似元素可能不是你的問題，除了性別以外，髮型的結果是最少的，其他隨機元素緩慢更新中"
command = "棉花娃稿"
mode = "image"
count = 0

[[shindan]]
id = "1186164"
title = "森罗重铸"
description = """
无论何人，无论何时，踏入此地者必须清楚：世界即将或者已经陷入危难之中，你现在的职责是作为引导者去找到解药并引领世界走上正规。仅此而已。
（ps：特别鸣谢群友笑子规对于函数部分的贡献）"""
command = "森罗重铸"
mode = "image"
count = 0

[[shindan]]
id = "828884"
title = "概括cp一生"
description = "不是什么好词"
command = "概括CP一生"
mode = "image"
count = 0

[[shindan]]
id = "1145946"
title = "你的欲望指数(自用)"
description = "代表你的欲望数值"
command = "欲望指数"
mode = "image"
count = 0

[[shindan]]
id = "864453"
title = "你的欲望"
description = "是雷达图，随便搞的"
command = "欲望雷达"
mode = "image"
count = 0

[[shindan]]
id = "1171805"
title = "几句歌词来概括/代餐你的CP或者OC"
description = """
歌词来自我的歌单，中/英文都会有
不限制性向性别（大概）
可能有些英文歌词比较惊悚，不是你家的口味再roll就好"""
command = "歌词概括我"
mode = "image"
count = 0

[[shindan]]
id = "1209847"
title = "少萝交友（正常版）"
description = "在寂寞的午夜寻找真爱吧"
command = "正常版少萝交友"
mode = "image"
count = 0

[[shindan]]
id = "637918"
title = "娘化穿越到异世界（正经）VER2"
description = """
用于做设定和跑团的正经版
人性简单理解是人性越低越理性越没有感情越不易动摇。不死性越高越不易被杀死。
||所有数据设定上是以一般人类的平均水平作为100（HPMP是10000）点的标准，熟练度及抗性除开加成100是极限
相对应的七美德和七宗罪不会同时出现，比如色欲和贞洁不会同时出现4||触手诊断ID:638952看看你的娘化能不能战胜你的触手化||不正经版ID:635902"""
command = "正经版娘化穿越"
mode = "image"
count = 0

[[shindan]]
id = "1154869"
title = "今天是什么武装JK"
description = """
战尘少女环境
最后更新2023.4.10"""
command = "武装JK"
mode = "image"
count = 0

[[shindan]]
id = "1135663"
title = "因為什麼而死，死前眼前浮現了什麼"
description = "什麼都可以測！歡迎代餐！"
command = "死前"
mode = "image"
count = 0

[[shindan]]
id = "834482"
title = "你死之前的眼底映照着些什么呢？"
description = """
不定时更新
【2018/12/19已更新】"""
command = "死前眼底"
mode = "image"
count = 0

[[shindan]]
id = "875491"
title = "每日三詞生成！"
description = "每天三詞，串成故事，或是挑一寫成段子！輸入cp／角色名皆可uwu"
command = "每日三词"
mode = "image"
count = 0

[[shindan]]
id = "486338"
title = "每日三题生成"
description = "每日三题生成器，可能中二向(′･ω･` )结果仅供参考"
command = "每日三题"
mode = "image"
count = 0

[[shindan]]
id = "837833"
title = "來畫單格漫畫吧！"
description = "抽一句台詞畫成單格漫！（部分台詞來至https://goo.gl/VfFc5R）"
command = "每日台词"
mode = "image"
count = 0

[[shindan]]
id = "1171473"
title = "屬於你的一句歌詞"
description = "一些常聽的歌的歌詞，可以代角色/自己也可以代cp"
command = "每日歌词"
mode = "image"
count = 0

[[shindan]]
id = "1151175"
title = "小波奇牌2023每日运势"
description = "小波奇牌2023每日运势"
command = "每日运势"
mode = "image"
count = 5

[[shindan]]
id = "394047"
title = "重口PLAY测试（无R18G内容只到R18）"
description = "输入CP或者角色名，自动生成2种道具或PLAY，祝各位脑洞得开心~结果每日替换！"
command = "每日重口"
mode = "image"
count = 0

[[shindan]]
id = "1118852"
title = "每日的题材(自用)"
description = "在畫了在畫了"
command = "每日题材"
mode = "image"
count = 0

[[shindan]]
id = "1031149"
title = "每日的梗之類的(全R18)"
description = "滿足自個兒性癖的玩意兒"
command = "每日黄梗"
mode = "image"
count = 0

[[shindan]]
id = "1213746"
title = "选择你的水晶冲突队友"
description = "吃瓜特供版"
command = "水晶冲突队友"
mode = "image"
count = 0

[[shindan]]
id = "1151559"
title = "水晶球预示了某一次的未来"
description = "在哪个地方发生了什么事呢(o^^o)♪"
command = "水晶球"
mode = "image"
count = 0

[[shindan]]
id = "1188702"
title = "汉末三国霸业（自用）"
description = "请在测试处根据“姓__名__字____”的格式(双引号内文字)输入姓名，例“姓王名丑字不帅”"
command = "汉末三国霸业"
mode = "image"
count = 0

[[shindan]]
id = "1117294"
title = "TA注视着你，说——"
description = """
被注视着的，注视着的，想逃走的。
*每日变化"""
command = "注视"
mode = "image"
count = 0

[[shindan]]
id = "767626"
title = "⚫如墜與你的深淵⚫"
description = """
「至今依然沉淪於思念中，無關乎心願。」 

⚜可能有致鬱效果⚜偏畫圖、試著寫文寫段子應該也是可以噠⚜( *´∀｀*)丿♡ 

⚜可以單就主題發揮⚜下面的文可以看看就好⚜

❀如果看完了文、喜歡、然後畫圖寫段子的話、我我我、我會愛你(´；ω；｀)❀

。°✿輸入名字開始測驗✿°。"""
command = "深渊"
mode = "image"
count = 0

[[shindan]]
id = "1136731"
title = "TA所渴望的是什么"
description = "TA潜意识中想要的是什么？（抽象表达）"
command = "渴望"
mode = "image"
count = 0

[[shindan]]
id = "1169928"
title = "激战2里你的OC"
description = "广受好评的MMORPG游戏"
command = "激战2"
mode = "image"
count = 0

[[shindan]]
id = "1214727"
title = "在火影里的你"
description = """
别名火影OC生成器哈哈
就是没法对输入字符串进行处理和判断这一点很不方便，，，
所以最好不要有姓氏，因为有可能摇到有血继的家族
不过“父母一方是血继家族成员但你随另一方的姓”设定就可以化解了hhh（
互相冲突的设定就自行取舍吧
另外忍者八项大部分项保底3（除了贤是2因为小堍贤二哈哈哈哈哈哈哈哈哈哈

总之bug致歉
另：头图是免费图库里找的一只卡！暗部卡帅帅的，很安心！
再另：搞火OC的咪务必扩我：3689735515！"""
command = "火影OC"
mode = "image"
count = 0

[[shindan]]
id = "1111492"
title = "測試你身上燃燒的火焰精神"
description = """
平安京里式神們性格各異，火焰也不盡相同。大人們是否真正瞭解自我，知曉自己心中燃燒的火焰是什麽顔色呢？
快來看看自己的火焰是什麽顔色，代表人物又是陰陽師里的哪位式神吧！"""
command = "火焰精神"
mode = "image"
count = 0

[[shindan]]
id = "833775"
title = "今天的靈感是什麼顏色？"
description = "把自己沒用上的靈感塞一塞，歡迎大家隨意使用。有顏色與文句，隨意解讀即可。不定時更新。根據以往的回饋好像滿是玻璃渣的樣子。"
command = "灵感颜色"
mode = "image"
count = 2

[[shindan]]
id = "1138653"
title = "神明的灵魂收藏品"
description = """
微猎奇向。
其实是“当神明创造你时”的迷惑版本（？"""
command = "灵魂收藏品"
mode = "image"
count = 0

[[shindan]]
id = "1136723"
title = "TA的爱人将是什么样的"
description = "TA的爱人拥有着什么样的元素，什么样的精神。"
command = "爱人"
mode = "image"
count = 0

[[shindan]]
id = "1140826"
title = "你的爱情表达是怎样的？"
description = "偏胃痛向 可以用来测试CP(还未完全修改好)"
command = "爱情表达"
mode = "image"
count = 0

[[shindan]]
id = "1116468"
title = "你的物品"
description = "/"
command = "物品"
mode = "image"
count = 0

[[shindan]]
id = "1200212"
title = "狂野之心里你的OC"
description = "卡普空要睡不着觉了"
command = "狂野之心OC"
mode = "image"
count = 0

[[shindan]]
id = "1182078"
title = "小猞猁代餐模拟器（私用）"
description = "rt"
command = "猞猁代餐"
mode = "image"
count = 0

[[shindan]]
id = "26646"
title = "你是什么样的猫？"
description = "对，没错，是老艾克出品的"
command = "猫"
mode = "image"
count = 0

[[shindan]]
id = "1175682"
title = "猫舍真实XP大揭秘~"
description = "nya~"
command = "猫舍XP"
mode = "image"
count = 0

[[shindan]]
id = "1149739"
title = "测测你的玄幻世界身份"
description = "测测你的玄幻世界身份"
command = "玄幻世界身份"
mode = "image"
count = 1

[[shindan]]
id = "1215599"
title = "如果你是瓦羅蘭的特務"
description = """
二創用
// 技能只有效果，沒有呈現方式（閃光能不能控制方向、反彈、穿牆）
// 不包含技能組的平衡問題（比如能不能被打掉、丟出去到產生效果的間隔時間、使用技能中途人物能不能移動或拿槍）"""
command = "瓦罗兰特务"
mode = "image"
count = 1

[[shindan]]
id = "1214626"
title = "来生成一款香水吧！"
description = "香型可能和生成的香调不太一样，但总之就是来看你信什么咯"
command = "生成香水"
mode = "image"
count = 0

[[shindan]]
id = "1204422"
title = "你生来注定是什么？"
description = "为我揭晓命运"
command = "生来注定"
mode = "image"
count = 0

[[shindan]]
id = "1188688"
title = "男友镜"
description = "某天 你误入了一个片神秘的森林，在一颗巨大的树下捡到了一面镜子，它那奇妙的魔力让你看到了..."
command = "男友镜"
mode = "image"
count = 0

[[shindan]]
id = "830216"
title = "你命中注定的男朋友是什么样的！👨🎤"
description = "99.82%的网友都参加了这个测试！！输入你的名字，我可以告诉你命中注定的男朋友是什么样子~"
command = "男朋友"
mode = "image"
count = 0

[[shindan]]
id = "22074"
title = "男版的自己!!"
description = "來測看看自己男生時長怎樣吧~~"
command = "男版自己"
mode = "image"
count = 0

[[shindan]]
id = "22295"
title = "男版的自己2"
description = "感謝大家支持《男版的自己》後來因為真的沖破12000了!!而且意外性的沖到30000多,,所以必須遵守約定做第二版p.s kuso性質升高注意"
command = "男版自己2"
mode = "image"
count = 0

[[shindan]]
id = "1087487"
title = "假如你作为男练习生参加了选秀节目"
description = "随便做着玩的"
command = "男练习生"
mode = "image"
count = 0

[[shindan]]
id = "1210032"
title = "R18慎入 作为男酮冒险者挑战触手迷宫副本的生成器"
description = """
你是一位挑战触手迷宫副本的【雄性】冒险者……在这个过程中遇到了怎样的故事呢？——填写你的名字测试看看吧！
注：同名字复占卜可以刷结果"""
command = "男酮副本"
mode = "image"
count = 0

[[shindan]]
id = "1204425"
title = "痒奴检查"
description = "检查痒奴的敏感度~"
command = "痒奴敏感度"
mode = "image"
count = 0

[[shindan]]
id = "1136153"
title = "占卜你的痴情程度"
description = "案例在一览表中输入 0～100 的数值，可以制作出用于“占卜出你的○○度”的小占卜"
command = "痴情程度"
mode = "image"
count = 0

[[shindan]]
id = "1166004"
title = "真·画手/文手转生[真女神转生角色cp抽签]"
description = """
这是一个全员都有可能出现的shindan，也就是说出现什么cp都是有可能的！！！BGBLGLGB人兽什么的都会有！！！！希望您不会被雷到˃ʍ˂洁癖严重的乖宝宝要小心使用哦

总之努力把能想到的真女神角色全都放进去了，要是之后想起来谁没放进去再补充叭，也许等之后真5F或者真6出来的话也会持续更新(做白日梦)
list主要是人类角色，还有少部分高人气恶魔，官方梦幻联动也算上了(例如真3狂热版和编年史的但丁雷道)

任务难度和奖励是我瞎写的，不用太在意~

如果能成功给您提供脑洞或者逗您开心就是我滴荣幸啦！"""
command = "真女神转生"
mode = "image"
count = 0

[[shindan]]
id = "860471"
title = "看看你在碧蓝航线中是怎样的船"
description = "纯属娱乐，如有雷同，绝不可能~"
command = "碧蓝航线"
mode = "image"
count = 0

[[shindan]]
id = "827223"
title = "你是一位怎样的神祇"
description = "你在其他界限里是掌管什麼的神呢（修正了bug 现在可以放心测试啦）"
command = "神"
mode = "image"
count = 0

[[shindan]]
id = "1140320"
title = "测测TA在神前川是什么样的居民吧！"
description = """
*还在持续更新中！*
（22.9.27）欢迎来到我家原创世界观神前川的测测！是一个日系现代架空世界，是没有魔法的普通21世纪都市呢。来看看TA在这个世界会是什么样的人生吧！
注：本测试包含天照联邦国的所有都市结果，不仅限于神前川都。"""
command = "神前川居民"
mode = "image"
count = 0

[[shindan]]
id = "1130986"
title = "神奇巫妖的生物積木"
description = """
如果未進行前作可能會有些莫名奇妙
（雖然前作也莫名奇妙"""
command = "神奇巫妖的生物"
mode = "image"
count = 0

[[shindan]]
id = "1215555"
title = "神秘商人的占卜——！"
description = "仅供参考，概不退换。"
command = "神秘商人"
mode = "image"
count = 0

[[shindan]]
id = "1115658"
title = "测测你是什么样的神秘学家"
description = "基于手游《重返未来1999》世界观设计，创作by千禧咖啡厅"
command = "神秘学家"
mode = "image"
count = 0

[[shindan]]
id = "827937"
title = "属于你的神必故事"
description = "有点沙雕 有点掉San 更多是莫名其妙的专属故事"
command = "神秘故事"
mode = "image"
count = 0

[[shindan]]
id = "1110841"
title = "あなたが選ぶ神アニメはこれだ！！"
description = "どうも！自称:アニメ評論家の主です！今回はあなたが選ぶ神アニメと言う事で各ジャンルに分けてアニメ作品を厳選させて頂きました！この診断では、あなたが神アニメと思う作品を診断します！ジャンルは以下の通りです。→ジャンプ部門、マガジン部門、ラブコメ部門、日常部門、ラノベ部門、ホラー/サスペンス部門、スポーツ部門、感動部門、ロボット部門など、この中から毎日1回あなたが神アニメと思う作品を1つ当てます！"
command = "神级动漫"
mode = "image"
count = 0

[[shindan]]
id = "1117639"
title = "神造汝之所添物"
description = "神明会不会是个糊涂鬼呢？唉嘿嘿嘿"
command = "神造物"
mode = "image"
count = 0

[[shindan]]
id = "792129"
title = "充满制作者私欲的随机外貌设定"
description = "好像勉强懂了一点！这是一个充满个人癖好的OC测试，会不定期更新【大概"
command = "私欲外貌"
mode = "image"
count = 0

[[shindan]]
id = "1079854"
title = "你在空中花园已经不当人了！"
description = "人不能，至少不应该"
command = "空中花园"
mode = "image"
count = 0

[[shindan]]
id = "1214977"
title = "如果你穿越进了异世界小黄油里"
description = "如果你穿越进了异世界小黄油里，你将会迎来怎样的命运呢~ଘ(੭ˊᵕˋ)੭🔮 ੈ✩（R18预警！！部分结局含有强制，人外，雌堕等性行为描写，请注意避雷！！主角与大部分人外均性别不定，可随喜好自行带入～）『目前更新了6种不同的剧情方向～更多结局还在更新中！』"
command = "穿越小黄油"
mode = "image"
count = 0

[[shindan]]
id = "1191719"
title = "穿越到奇幻异世界后！你成为了……"
description = "穿越到奇幻异世界后！你成为了……"
command = "穿越异世界"
mode = "image"
count = 0

[[shindan]]
id = "1204491"
title = "穿越成兽人"
description = """
经过彻夜的工作，神情恍惚的你最终睡了过去。
再度睁眼，身边已经不再是自己熟悉的世界。
在新的世界里，你会遭遇什么呢？"""
command = "穿越成兽人"
mode = "image"
count = 0

[[shindan]]
id = "1204547"
title = "检测穿越到绒球时你会变成什么生物！"
description = """
重奏生物非常多样也非常的可爱！
那么穿越到绒球的你会成为哪一种生物呢？"""
command = "穿越绒球"
mode = "image"
count = 0

[[shindan]]
id = "1111932"
title = "你OC的童年是什么样的？"
description = "瞎做的，瞎玩玩吧"
command = "童年"
mode = "image"
count = 0

[[shindan]]
id = "850187"
title = "如果你有第二人格，那麼他會是什麼模樣？"
description = "DID是一種很嚴重的精神疾病。很多人都不知道該病是否真實存在，以及患者的生活究竟為什麼樣子。如果你是DID患者，那麼另一個你是誰？"
command = "第二人格"
mode = "image"
count = 0

[[shindan]]
id = "1175225"
title = "今天哪個男/女嘉賓會抱到哪隻菇菇呢？"
description = """
又一個all冒cp產生器（but菇菇or菇娘隨機出現版）
所有角色無論男女皆攻，冒險家固定右位
其他cp請勿代餐"""
command = "第五人格CP"
mode = "image"
count = 0

[[shindan]]
id = "1175220"
title = "今天菇菇會寵幸哪位男/女嘉賓呢？"
description = """
一個all冒cp產生器（莊園角色純享版）
所有角色無論男女皆攻，冒險家固定右位
其他cp請勿代餐"""
command = "第五人格CP2"
mode = "image"
count = 0

[[shindan]]
id = "1212872"
title = "欢迎来到米德加尔德大陆！"
description = "开始你的魔法体验吧！"
command = "米德加尔德"
mode = "image"
count = 0

[[shindan]]
id = "1213052"
title = "什么类型学代码跟你最配呢？！"
description = """
可以输入名字也可以输入代码（如果不限制字数的话）
包含MBTI，九型人格，本能副型，tritype，soc，大五人格，态度类型，时间论，气质类型，阵营九宫格~
因为九型人格跟tri是分开的，所以会有主型侧翼跟tri不一样的情况，对不起
每天都会变"""
command = "类型学代码"
mode = "image"
count = 0

[[shindan]]
id = "1213050"
title = "你的类型学代码是……？！"
description = """
包含MBTI，九型人格，本能副型，tritype，soc，大五人格，态度类型，时间论，气质类型，阵营九宫格~
因为九型人格跟tri是分开的，所以会有主型侧翼跟tri不一样的情况，对不起
每天都会变"""
command = "类型学代码2"
mode = "image"
count = 0

[[shindan]]
id = "851039"
title = "精神污染·每日三题"
description = "写cp文供梗，其实并不算很精神污染。"
command = "精神污染三梗"
mode = "image"
count = 0

[[shindan]]
id = "403243"
title = "精神病院测试"
description = "“迷失在梦里的天才” 某一天被送进了精神病院的你有着怎样的才华呢？（不定期更新）"
command = "精神病院"
mode = "image"
count = 0

[[shindan]]
id = "934511"
title = "一个纷乱的梦。（融合了许多游戏梗"
description = """
新人加入，多多指教_(:з」∠)_
在这里做一个梦似乎也不错……
融合了UNDERTALE、阴阳师、宝可梦等各种梗
逻辑可能混乱不要在意，反正梦就是没有逻辑的"""
command = "纷乱的梦"
mode = "image"
count = 0

[[shindan]]
id = "1138197"
title = "人生終有一死"
description = "意外真多"
command = "终有一死"
mode = "image"
count = 0

[[shindan]]
id = "1137262"
title = "ta的结局是什么"
description = "做完发现好多内容也不能算结局，就当做是生活中的某个片段吧。挺多样的，欢迎来代餐😋"
command = "结局"
mode = "image"
count = 0

[[shindan]]
id = "1117883"
title = "★绘画主题★"
description = "可以在没有绘画灵感的时候用哦！_(•̀ω•́ 」∠)_"
command = "绘画主题"
mode = "image"
count = 0

[[shindan]]
id = "1124540"
title = "随机绘画元素"
description = "结果会每次改变"
command = "绘画元素"
mode = "image"
count = 0

[[shindan]]
id = "832219"
title = "每日繪畫關鍵詞生成器(我流)"
description = """
比較隨個人喜好啦XD還在完善中，想到什麼就不定時更新。
都是些比較有象征意義和畫面感的東西，應該比較好畫【？】比較幻想風。"""
command = "绘画关键词"
mode = "image"
count = 0

[[shindan]]
id = "1131364"
title = "自用日绘随机关键字"
description = "给自己日绘所用！"
command = "绘画关键词2"
mode = "image"
count = 0

[[shindan]]
id = "1126607"
title = "绘画关键词生成器"
description = "输入名字来获取随机的绘画关键词吧！"
command = "绘画关键词3"
mode = "image"
count = 0

[[shindan]]
id = "1210449"
title = "繪畫題目發想2：角色塑造產生器"
description = "繪畫題目發想第二彈，參數有所調整，主要輔助發想角色的身分特質，若出現互相矛盾的條件就請自行取捨或發揮想像力XD，"
command = "绘画角色"
mode = "image"
count = 0

[[shindan]]
id = "1136113"
title = "給你一權"
description = """
（胡思亂想的產物
（背景制作（請填領導者名稱
（中世紀普通人的故事？"""
command = "给我一权"
mode = "image"
count = 0

[[shindan]]
id = "20880"
title = "其實這才是最適合你的綽號"
description = "其實這才是最適合你的綽號,交給全知萬能的神吧!!別反抗!!沒有別的意見!!"
command = "绰号"
mode = "image"
count = 0

[[shindan]]
id = "1141092"
title = "网名生成器"
description = """
娱乐向
常更新
全部都是乱打的，所以可能会有语病"""
command = "网名"
mode = "image"
count = 0

[[shindan]]
id = "764124"
title = "罪囚"
description = """
『又是犯了罪的囚人啊啊....』

監獄聚集了各式各樣的人感嘆著新來的一批罪囚，各懷著自己的心思。

而你，又是因何而到來呢?"""
command = "罪囚"
mode = "image"
count = 0

[[shindan]]
id = "1141399"
title = "美利坚非裔生存指南"
description = "争取活下去吧！"
command = "美利坚非裔生存指南"
mode = "image"
count = 0

[[shindan]]
id = "1151952"
title = "你今天的灵魂超美味"
description = "写着写着突然饿了 吃点什么好呢 (* ´ω`*)"
command = "美味灵魂"
mode = "image"
count = 0

[[shindan]]
id = "1204415"
title = "阿美莉卡jc大乐透"
description = "输入ID以获取您的评估报告"
command = "美国警察"
mode = "image"
count = 0

[[shindan]]
id = "828727"
title = "会成为怎样的美少女偶像呢"
description = "小偶像是人类救星啊！大家都最最可爱啦！"
command = "美少女偶像"
mode = "image"
count = 0

[[shindan]]
id = "1204718"
title = "测测你在群像战斗动画中会是什么样的角色"
description = "测测你在群像战斗动画中会是什么样的角色。每小时会变化一次结果"
command = "群像战斗动画OC"
mode = "image"
count = 0

[[shindan]]
id = "16775"
title = "你未來的老公/婆"
description = "就是你未來的老公/婆"
command = "老公或老婆"
mode = "image"
count = 0

[[shindan]]
id = "1160185"
title = ">职业主题<角色生成器"
description = "职业和特殊元素主题的生成器，推荐用来提供角色设计的灵感！每日更换"
command = "职业主题角色生成"
mode = "image"
count = 0

[[shindan]]
id = "1176240"
title = "职业连连看"
description = "写产品用，输入产品姓名进行职业配对。"
command = "职业连连看"
mode = "image"
count = 0

[[shindan]]
id = "926484"
title = "頗糟糕的肉文梗產生器"
description = "祝食用愉快"
command = "肉文梗"
mode = "image"
count = 0

[[shindan]]
id = "1129719"
title = "肢（枝）體拼接"
description = """
頭好痛的巫妖在醉酒情況下拼出的東西
（之後會有一個細化版的"""
command = "肢体拼接"
mode = "image"
count = 0

[[shindan]]
id = "384482"
title = "互動命題(*´ﾟ∀ﾟ)从(ﾟωﾟ｀*)CP腦洞關鍵字"
description = "適用於微博/推特/噗浪的互動遊戲,輸入CP名進行測試,有糖有虐有脫線(ゝω・)キラッ☆ 単人/多人用互動命題ID:383328"
command = "脑洞关键词"
mode = "image"
count = 17

[[shindan]]
id = "35462"
title = "我的自傳"
description = "你的自傳診斷."
command = "自传"
mode = "image"
count = 0

[[shindan]]
id = "24700"
title = "你的自傳"
description = "你的自傳裡記載了你的一生"
command = "自传2"
mode = "image"
count = 1

[[shindan]]
id = "806227"
title = "✦自戕者✧"
description = """
他們說自我了斷是一種罪孽 。
我的靈魂在業障輪迴之中掙扎翻騰，永世不得超生。"""
command = "自杀"
mode = "image"
count = 0

[[shindan]]
id = "788053"
title = "自设世界观的人设生成器"
description = "用来捏自设用……"
command = "自设人设生成"
mode = "image"
count = 0

[[shindan]]
id = "1141554"
title = "占卜你的色情度"
description = "案例在一览表中输入 0～100 的数值，可以制作出用于“占卜出你的○○度”的小占卜"
command = "色情度"
mode = "image"
count = 0

[[shindan]]
id = "1079091"
title = "【R18創作靈感】色色的10連特選卡片池"
description = "來個10連抽，獲得R18創作題材卡片❗本池為成人向，不含血腥或過度傷害內容，SM有。"
command = "色色十连"
mode = "image"
count = 0

[[shindan]]
id = "1216467"
title = "色色的战败生成器(虽然没有战)1.0"
description = "整活整出来的，不要太在意细节。后面可能会更新，可能包含女同()"
command = "色色战败"
mode = "image"
count = 0

[[shindan]]
id = "1099294"
title = "色色的cp互动题目大赏"
description = "没有sex和r18的cp就是一堆散沙！"
command = "色色话题"
mode = "image"
count = 0

[[shindan]]
id = "595068"
title = "Fate 英霊召喚"
description = """
あなたのサーヴァントはなんでしょう
ネタバレ含む
全員いるわけじゃないです"""
command = "英灵召唤"
mode = "image"
count = 0

[[shindan]]
id = "161794"
title = "你成为英灵参加圣杯战争的结果（修正加强版）"
description = "无"
command = "英灵圣杯战争"
mode = "image"
count = 0

[[shindan]]
id = "346290"
title = "在Fate中你会是哪个英灵的职阶"
description = "正统向Fate英灵系统测试"
command = "英灵职阶"
mode = "image"
count = 0

[[shindan]]
id = "1117802"
title = "每日荣格八维+九型人格"
description = "获取你的每日人格类型。（注：不包括荣格八维未分化情况，只有已分化的处于两个功能中间的功能）"
command = "荣格八维与九型"
mode = "image"
count = 0

[[shindan]]
id = "1188679"
title = "下一站，驚喜！<機會>"
description = "恭喜你獲得一項改變命運的機會"
command = "获得机会"
mode = "image"
count = 0

[[shindan]]
id = "1171145"
title = "属于你的菊科花卉"
description = "孩子不懂事写着玩的"
command = "菊科花卉"
mode = "image"
count = 0

[[shindan]]
id = "799582"
title = "每日虐梗"
description = """
如題，以下所出現的題目皆是虐梗！
當然要嘗試虐題寫甜也是可以的！"""
command = "虐梗"
mode = "image"
count = 0

[[shindan]]
id = "1058945"
title = "你的表裏人格一致嗎?"
description = """
陰陽世界，不止於黑白，每位式神都有著獨一無二的鮮明個性。好比蟬冰雪女一面與霜雪為伴，一面期待春日暖陽。
立體多面的各位陰陽師大人們與平安京中的誰更相似呢？掃地工準備了妙計幫助各位大人發現平安京中的另一個自己噢！快來試試看吧~"""
command = "表里人格"
mode = "image"
count = 0

[[shindan]]
id = "1141123"
title = "當你做完噩夢之後醒來之後你看到了——"
description = """
比較意識流，看看就好（點頭）
可能有點g（目移）"""
command = "被噩梦惊醒"
mode = "image"
count = 0

[[shindan]]
id = "1134236"
title = "每日西幻限定人设"
description = """
西幻文系列～请输入名字吧！
有想法就会更新
2023/10/26 已更新"""
command = "西幻人设"
mode = "image"
count = 0

[[shindan]]
id = "1135896"
title = "西幻故事生成器"
description = """
可以製作故事、可自用、代餐或者搞if線
各種角色都有可能
死亡要素有、大多虛構元素
有靈感的時候會不斷更新"""
command = "西幻故事"
mode = "image"
count = 0

[[shindan]]
id = "1201928"
title = "觀察者協會✧ ✦內部人員"
description = """
歡迎來到觀察者協會，你是新報到的成員吧？手冊和裝備都確認完畢了嗎？那些「東西」可不會輕易放過你，多做些準備總是對的。

（原創世界觀，整理中，之後會新增選項）"""
command = "观察者协会"
mode = "image"
count = 0

[[shindan]]
id = "1084644"
title = "角之国某学院学生（自己填写的）档案"
description = "welcome"
command = "角之国学生"
mode = "image"
count = 0

[[shindan]]
id = "940214"
title = "✦角之旅✧"
description = """
『吾乃角之子民。』
犄角屬於工匠的種族，他們的血脈在大陸戰爭結束後四散各方、技藝一脈相承；隨世事分合興衰，頭戴犄角的人們，又再一次踏上先祖顛沛的道路……"""
command = "角之旅"
mode = "image"
count = 0

[[shindan]]
id = "1160220"
title = "给你的角色的新故事"
description = """
给画图写文提供故事情节灵感用？微西幻背景（每次结果都变化）
主要要素：角色·地点·做什么·付出代价·结果
【可能时不时更新】"""
command = "角色故事"
mode = "image"
count = 0

[[shindan]]
id = "1218947"
title = "主题词+身份的角色灵感"
description = "个人用来做画猜词库的，顺便做人设灵感"
command = "角色灵感"
mode = "image"
count = 0

[[shindan]]
id = "1124830"
title = "每日随机角色词汇生成器 人设向"
description = "相对比较完整的人设词汇生成器。可用作角色设计练习。会持续更新词库。"
command = "角色词汇"
mode = "image"
count = 0

[[shindan]]
id = "1135679"
title = "認識扭曲者"
description = """
第三層的特產，
因劇烈的心理變化，
而在死亡後形成的怪物。
（多由二層的倖存者變成
（由於是認識類的，種類會比較少"""
command = "认识扭曲"
mode = "image"
count = 0

[[shindan]]
id = "1189616"
title = "为你写一首诗"
description = "意识流产物，乱七八糟的，前言不搭后语，有嗑药元素并且负能量比较多（问就是主包是个嗑疯了的精神病（摊手"
command = "诗"
mode = "image"
count = 1

[[shindan]]
id = "1133449"
title = "調教木馬desu⭐"
description = """
⭐R18⭐
來測測看坐到什麼樣的木馬上吧！"""
command = "调教木马"
mode = "image"
count = 0

[[shindan]]
id = "572991"
title = "ランダムカラーパレット"
description = "診断結果の5色の色のみを使ってイラストを描いてみましょう（使用ジェネレーター：https://coolors.co/　←自分でもカラーパレットを作れます）"
command = "调色板"
mode = "image"
count = 0

[[shindan]]
id = "1117719"
title = "豪门宇宙里的你"
description = "谋杀之谜游戏《豪门惊情》系列相关，剧透注意！仅供娱乐，可能会陆续更新。结果会每天变化～"
command = "豪门宇宙"
mode = "image"
count = 0

[[shindan]]
id = "1161339"
title = "貼身衣物"
description = "貼身衣物有了意識"
command = "贴身衣物"
mode = "image"
count = 0

[[shindan]]
id = "23446"
title = "不囉唆!!你作什麼職業最賺錢!!"
description = "想賺錢嗎!? 聽我的就對了啊!! (感謝上個測驗高人氣,但我實在沒梗了XXD 玩看看吧~"
command = "赚钱职业"
mode = "image"
count = 0

[[shindan]]
id = "1085569"
title = "你适合什么样的赛马娘怪文书？"
description = """
怪就是好！好就是怪！怪文书一天不看难受，看了难受一天。
找怪文书请看：https://ngabbs.com/read.php?tid=260892..."""
command = "赛马娘怪文书"
mode = "image"
count = 0

[[shindan]]
id = "1215367"
title = "起个名字吧"
description = """
同时包含了西式名字、中式名字和日式名字的生成器
因为是字库随便摇的字所以会有一些离谱结果
姓氏直接取的百家姓，你没见过的复姓不是日本姓！"""
command = "起名"
mode = "image"
count = 0

[[shindan]]
id = "464514"
title = "超能力測試"
description = "資料來自科學超電磁炮"
command = "超能力"
mode = "image"
count = 0

[[shindan]]
id = "1134628"
title = "您覺醒的超能力診斷"
description = "[創作企劃專用]Ortus公司提供的超能力診斷資訊"
command = "超能力2"
mode = "image"
count = 1

[[shindan]]
id = "571574"
title = "あなたの身体の年齢診断"
description = "あなたの身体中の年齢を教えます"
command = "身体年龄"
mode = "image"
count = 0

[[shindan]]
id = "1178655"
title = "做一个角色（仅身体特征）"
description = """
祝玩得愉快

仅限人类身体，二次元向*"""
command = "身体特征"
mode = "image"
count = 0

[[shindan]]
id = "1161875"
title = "身體黏土"
description = "身體被塑形是甚麼體驗"
command = "身体黏土"
mode = "image"
count = 0

[[shindan]]
id = "985322"
title = "あなたのスリーサイズと身長と体重（女の子編）"
description = "あなたのスリーサイズと身長と体重を暴露します。男性がやるとあなたがもし女の子だったらどんな感じだったのかわかります。"
command = "身高体重"
mode = "image"
count = 0

[[shindan]]
id = "1159628"
title = "开大车专用梗生产器"
description = "仅限于各种车文的梗，都是本人喜欢的，祝食用愉快"
command = "车文梗"
mode = "image"
count = 0

[[shindan]]
id = "1117762"
title = "與你最為契合的車次少女是......？"
description = "車次少女們一個個都有她們獨特的個性，一個個都讓人壓抑不住婆的衝動！究竟哪個車次少女是你命定中的老婆呢？趕快來測試看看吧！"
command = "车次少女"
mode = "image"
count = 0

[[shindan]]
id = "1161249"
title = "轉化詛咒"
description = "被詛咒了"
command = "转化诅咒"
mode = "image"
count = 0

[[shindan]]
id = "1097999"
title = "关于你转学到如恩学院这件事"
description = "欢迎来到如恩学院"
command = "转学如恩学院"
mode = "image"
count = 0

[[shindan]]
id = "624261"
title = "JOJO的世界，不去看看吗？"
description = "充满了恶意的测试呢。"
command = "转生JOJO"
mode = "image"
count = 0

[[shindan]]
id = "1202290"
title = "关于我转生成为兽人的那件事"
description = """
关于我转生成为兽人的那件事v1.0
可能后续还会更新，欢迎提意见"""
command = "转生兽人"
mode = "image"
count = 0

[[shindan]]
id = "1200734"
title = "异世界转生（兽耳篇）"
description = "r8r8"
command = "转生兽耳"
mode = "image"
count = 0

[[shindan]]
id = "1052778"
title = "あなたは原神世界に生まれ落ちました"
description = """
あなたは原神世界に生まれ落ちました。さて、あなたはどんな神の目を持ち、どんな武器を使用するのか。どこから来て、どこに所属していて、誰と交友関係があるのか。
もしかしたら神の目は持っておらず、出身地が分からないかもしれません。少し覗いていきませんか？

※結果は毎日変わります。"""
command = "转生原神"
mode = "image"
count = 0

[[shindan]]
id = "1189753"
title = "当你转生到异世界你是什么样子"
description = "阿巴阿巴"
command = "转生异世界"
mode = "image"
count = 0

[[shindan]]
id = "1204703"
title = "你会转生为什么种族的恶魔"
description = "你的意识属性，抗性，技能适应性，和下一个学会的技能"
command = "转生恶魔"
mode = "image"
count = 0

[[shindan]]
id = "1204676"
title = "转生艾欧泽亚，你是什么身份"
description = "半夜12点，你正在玩FF14，外面正在打雷下雨，突然一道雷电劈中你的房间，你穿越到艾欧泽亚"
command = "转生艾欧泽亚"
mode = "image"
count = 0

[[shindan]]
id = "1215014"
title = "转生鸡窝托斯当学生吧！"
description = "第一次搞这种，不喜勿喷()整到最后没有可改变值了就打算用随机数了，自己估一下大小吧()还有他们的三年二年生没整明白就没写"
command = "转生鸡窝托斯"
mode = "image"
count = 0

[[shindan]]
id = "1207229"
title = "【边狱巴士】边狱巴士pa OC随机生成器"
description = """
-该生成器主要用于提供绘画灵感，不包含细节信息（技能类型等）
-人格随机列表中不包含部分性质特殊的人格（如脑叶支部幸存者、裴廓德号船长、准执柄者等），且考虑到协会收尾人服饰设计的统一性，不对协会收尾人人格进行基于科的详细划分
-E.G.O列表包含了脑叶公司中登场的E.G.O装备。出现同时在两部作品中登场的E.G.O或是存在整体设计风格相似的原版与亚种时，列表将优先收录在边狱巴士中实装的E.G.O
-主要是自用，不一定会及时更新"""
command = "边狱巴士OC"
mode = "image"
count = 0

[[shindan]]
id = "829339"
title = "你的过去"
description = "一些还没被打碎的东西。"
command = "过去"
mode = "image"
count = 0

[[shindan]]
id = "1192462"
title = "尝试制作的连缘风格称号生成器"
description = "有任何脸要素吗"
command = "连缘风格称号"
mode = "image"
count = 0

[[shindan]]
id = "1206003"
title = "迷宫饭oc生成"
description = "迷宫饭世界背景加dnd等"
command = "迷宫饭OC"
mode = "image"
count = 0

[[shindan]]
id = "830408"
title = "预测你的追星结局"
description = "追星的结局，也许是另一个开始"
command = "追星结局"
mode = "image"
count = 0

[[shindan]]
id = "1203802"
title = "VENTANGLE随机追猎者"
description = "VENTANGLE随机追猎者 中文"
command = "追猎者"
mode = "image"
count = 0

[[shindan]]
id = "1116526"
title = "【自家OC】透过镜片去观测吧"
description = "自娱自乐向。毫无意义。不稳。oc世界大缝合。"
command = "透过镜片"
mode = "image"
count = 0

[[shindan]]
id = "1127247"
title = "逝者的履歷"
description = """
事物死亡後，如同分類一樣各有去處。
身，為物之載體，為構成萬物之軀。
心，又稱意志，萬事萬物之本性。
靈，為物之流體，為無窮無盡之心。
物為何擁有其型與概念？
無可知曉。
///////////////（上邊只是感慨，與內容無關）
穿越（忘記要打什麼了）
（已將過激言論剃除）
（怕網爆（x）"""
command = "逝者履历"
mode = "image"
count = 0

[[shindan]]
id = "1129921"
title = "道诡人生"
description = """
测试你在道诡异仙中的身份吧！
v1.05，总之就是还在更新啦
PS:目前正在制作完整版的道诡人生重开模拟器，敬请期待"""
command = "道诡人生"
mode = "image"
count = 0

[[shindan]]
id = "833754"
title = "你一生最大的遗憾"
description = "测测你或某角色一生最大的遗憾～"
command = "遗憾"
mode = "image"
count = 0

[[shindan]]
id = "942140"
title = "配色生成机器"
description = "输入任何字符随机生成配色"
command = "配色"
mode = "image"
count = 0

[[shindan]]
id = "790825"
title = "奇妙的配色环"
description = "快乐配色"
command = "配色环"
mode = "image"
count = 0

[[shindan]]
id = "1204403"
title = "预测你重生后的未来"
description = "一切皆有可能"
command = "重生后的未来"
mode = "image"
count = 0

[[shindan]]
id = "1204427"
title = "重生之我在幻想乡..."
description = "这个幻想乡太乱...."
command = "重生幻想乡"
mode = "image"
count = 0

[[shindan]]
id = "1198177"
title = "【藍色系/繁體】你在他人的重要程度？"
description = """
請填取他/她的名字！
主要的項目有重要、好感、厭惡、不喜不厭。"""
command = "重要程度"
mode = "image"
count = 0

[[shindan]]
id = "1122309"
title = "你们会在何时何地重逢"
description = "总之就是想见面"
command = "重逢"
mode = "image"
count = 0

[[shindan]]
id = "16073"
title = "测试你的野心beta1"
description = "很准哦！"
command = "野心"
mode = "image"
count = 0

[[shindan]]
id = "1213041"
title = "钢机占卜"
description = """
来到第一个轮回世界后，发现周围似乎是少林寺
主神给你的任务是：【打倒东方不败】
正当你觉得不过是武侠世界的时候，方丈把你拉到了机库
指着一台机体说：你就用这台机体参战吧
合着是这么个东方不败啊！
那么你的机体是什么呢"""
command = "钢机"
mode = "image"
count = 0

[[shindan]]
id = "1134897"
title = "鎧甲人體組裝"
description = "滿地屍骸的古戰場，毀壞的軀體向著活著的人彙集而去"
command = "铠甲人体组装"
mode = "image"
count = 0

[[shindan]]
id = "829595"
title = "如何铸造你的灵魂"
description = "“坩埚和一些乱七八糟的药水儿”"
command = "铸造灵魂"
mode = "image"
count = 0

[[shindan]]
id = "1098388"
title = "假如你闯入了ABO世界的内娱"
description = "无聊产物"
command = "闯入ABO内娱"
mode = "image"
count = 0

[[shindan]]
id = "1132629"
title = "閻魔之目，洞悉你的人格善惡"
description = """
光芒所到之處，必有陰影的存在，人心亦是如此。

大人們是否足夠了解自己的內心❓

傳聞中閻魔之目能洞悉一切靈魂，大夜摩天閻魔大人將直視大人的內心，#用5條標籤表示你的善惡成分 揭開你心中的真實一面！"""
command = "阎魔之目"
mode = "image"
count = 0

[[shindan]]
id = "1152257"
title = "小阿卡纳适应性"
description = "在旅行中，阿卡夏之钥系统会判定同行者的适应性。请跟虚空精灵成为朋友吧。"
command = "阿卡纳适应性"
mode = "image"
count = 0

[[shindan]]
id = "933189"
title = "你在陌生而又熟悉的世界中会是怎样的人"
description = """
在一个有着现实的人类社会，背地里却隐藏了无数深秘的世界，你会是个什么样的角色？

PS：能力数据的说明：
〔1为低于正常人，2为正常人类，3为通常的人类的极限（魔法师、学习气功的武者、打开基因限制等等的存在除外），4、5、6分别超越上一级的能力值，7为无法评估或者超出能力值检测最大计算量，但是外貌、智商、意志、幸运除外，它们即使是正常人类也能达到3以上的数值。〕"""
command = "陌生而又熟悉的世界"
mode = "image"
count = 0

[[shindan]]
id = "936211"
title = "当神明创造他时"
description = """
微猎奇向（大概）有些无趣。过长警告。
灵感来源于“上帝制造你时加了什么东西？”"""
command = "降临"
mode = "image"
count = 0

[[shindan]]
id = "1200270"
title = "【DC脑洞用】随机角色扮演模拟器"
description = "脑洞产物。来一次扮演随机dc角色，有随机任务和技能的穿越吧。一个名字每次重测会不一样，可以多试几次"
command = "随机DC角色"
mode = "image"
count = 0

[[shindan]]
id = "1129297"
title = "创造者之旅随机事件"
description = """
创造者之旅企划用，随机事件抽取，抽取事件后创作一个满足完成度要求的作品可以获得随机道具，事件描述给出的行动是参考行动，玩家可以不按照事件给出的示例，按自己心意对事件做出反应。
有些随机事件完成后会开启后续的故事线，可以携抽取的截图和完成的截图找企划主继续后续故事。
在企划开启期间会随时更新新的随机事件。"""
command = "随机事件"
mode = "image"
count = 0

[[shindan]]
id = "1145108"
title = "【作业自用】"
description = "随机生成作业用大转盘"
command = "随机作业"
mode = "image"
count = 0

[[shindan]]
id = "778274"
title = "★随机外貌设定★"
description = "基本的外貌设定，身高肤色头发眼睛细节特征等。每日更新。"
command = "随机外貌"
mode = "image"
count = 0

[[shindan]]
id = "1138236"
title = "你的SCHOOL学员"
description = """
我家主世界观的随机学生捏造。
请随意游玩。
随时更新。"""
command = "随机学员"
mode = "image"
count = 0

[[shindan]]
id = "1138193"
title = "對話應用（自嗨"
description = """
（後面的（）請自行帶入
（與現實無任何關聯
（沒什麼用"""
command = "随机对话"
mode = "image"
count = 0

[[shindan]]
id = "1197333"
title = "以闪亮之名随机搭配挑战"
description = """
使用后发到公开社交平台时需要在可以直接看见的地方注明作者和分享链接，禁止用来给自己引流（比如点赞关注私信发链接）。
如有建议请联系qq：2201853828
b站：森雀-official
小红书：BY子"""
command = "随机搭配"
mode = "image"
count = 0

[[shindan]]
id = "1215022"
title = "oi，随机数！"
description = """
只有随机数的一个占
包含了各类战斗和心理数值等等
oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！oi，随机数！"""
command = "随机数"
mode = "image"
count = 0

[[shindan]]
id = "1117756"
title = "占卜你的○○度111"
description = "案例在一览表中输入 0～100 的数值，可以制作出用于“占卜出你的○○度”的小占卜"
command = "随机数0-100"
mode = "image"
count = 0

[[shindan]]
id = "1073636"
title = "隨機數值（雷達圖）"
description = """
［測試］隨便出點數據—雷達圖
STR:Strength
INT:Intelligence
APP:Appearance 
DEX:Dexterity
EDU:Education
和TRPG沒有任何關係，只是想不到要用甚麼"""
command = "随机数值"
mode = "image"
count = 0

[[shindan]]
id = "1127812"
title = "【吃饭模拟器】随机生成（1.0）"
description = '''
来代餐吧！！！

出于已被冻结的骰娘「Leibot_0401」。

"输入ROLL或任意字词使用！"

可以改动！可以当OC！欢迎加群分享！
【作者OC唠嗑QQ群：731638039】
【版本：1.0】'''
command = "随机生成"
mode = "image"
count = 0

[[shindan]]
id = "1215036"
title = "随机色卡生成器"
description = """
颜色较少，伪随机，有不和谐色的可能性，但你可以使用↓

通过百度网盘分享的文件：随机色卡生成器
链接：https://pan.baidu.com/s/1KPm_3Rb_zFr_u6Jg... 
提取码：fzyq 
--来自百度网盘超级会员V5的分享

↑（一个仅支持电脑版的随机色卡生成器，分享链接包含了exe文件和C++源代码）
（人格担保没有病毒！！！）
（我自己写的代码所以可以点个小红心支持手艺人吗qwq）"""
command = "随机色卡"
mode = "image"
count = 0

[[shindan]]
id = "1215639"
title = "繪圖創作 / 隨機色票挑戰"
description = """
自用
用指定的色票和比例創作角色
イラスト創作 / 色チャレンジ 中文版"""
command = "随机色票"
mode = "image"
count = 0

[[shindan]]
id = "1184080"
title = "隨機表符（完整）8種"
description = """
由表情符號贊助提供的靈感池（×）
抽完後可以自己更改要素，有部分手殘刪除、重複（但基本都是打散的）和無法打出的表符
【無法避免您抽到討厭的要素】"""
command = "随机表情"
mode = "image"
count = 0

[[shindan]]
id = "1137948"
title = "随机三个emoji"
description = "/"
command = "随机表情2"
mode = "image"
count = 0

[[shindan]]
id = "1158925"
title = "随机角色生成"
description = "人设练习用，刷到矛盾的觉得画不出来的重新刷一遍"
command = "随机角色"
mode = "image"
count = 0

[[shindan]]
id = "1186057"
title = "纯雪自用的骰子卡牌小游戏角色卡随机器——每日版"
description = "纯雪针对六面骰子设计的小游戏，一个骰子就能玩！规则简单！"
command = "随机角色卡"
mode = "image"
count = 0

[[shindan]]
id = "1164307"
title = "随机角色设计题目"
description = """
自用，角色设计练习随机元素，元素冲突的地方自己取舍合理化。
微博@kicel_"""
command = "随机角色设计"
mode = "image"
count = 0

[[shindan]]
id = "1123356"
title = "隨機顏色"
description = "隨機的顏色"
command = "随机颜色"
mode = "image"
count = 0

[[shindan]]
id = "1183346"
title = "随机骰子"
description = """
这次是正式版！趁着现在有热情多做一些哈哈哈，以后可能还会持续更新的～
（碎碎念：这玩意儿可比rpgmaker好用多了）"""
command = "随机骰子"
mode = "image"
count = 0

[[shindan]]
id = "829302"
title = "你的隐藏气味"
description = "你的隐藏气味是什么"
command = "隐藏气味"
mode = "image"
count = 0

[[shindan]]
id = "1060802"
title = "test111111111"
description = "test"
command = "雷达图"
mode = "image"
count = 0

[[shindan]]
id = "944833"
title = "霍格華茲聊天室"
description = """
今天在霍格華茲聊天室
自我流D20數值解釋
１：還活著就是奇蹟
２～３：勉強維生
４～５：弱
６～７：不擅長
８～９：比平均值稍弱
10～11：正常人/平均值
12～13：比平均值強
14～15：可代表出場比賽選手
16～17：世界冠軍
18～19：神子女（？） 20：神"""
command = "霍格沃兹聊天室"
mode = "image"
count = 0

[[shindan]]
id = "957935"
title = "霍格沃茨毕业生的生平与谎言"
description = "会魔法的你离开霍格沃茨之后又会过上什么样的人生？是在阿兹卡班坐穿牢底，还是在魔法部勤恳工作走上人生巅峰？"
command = "霍格沃茨毕业生"
mode = "image"
count = 0

[[shindan]]
id = "1122083"
title = "霾家人生成器"
description = "可以加入这个家吗"
command = "霾家人生"
mode = "image"
count = 0

[[shindan]]
id = "1139589"
title = "非人类学院角色卡池"
description = "来这里抽取一个有超能力的学生，看看你的手气！"
command = "非人类学院"
mode = "image"
count = 0

[[shindan]]
id = "1104123"
title = "假如你打造了一个韩娱9人男团"
description = "无聊产物 纯属娱乐"
command = "韩娱男团"
mode = "image"
count = 0

[[shindan]]
id = "1156092"
title = "预测一个有可能实现也有可能不实现的某人的未来"
description = "预测一个有可能实现也有可能不实现的某人的未来"
command = "预测未来"
mode = "image"
count = 0

[[shindan]]
id = "1131718"
title = "预测你的未来qq"
description = "使用三个一览表可生成较复杂文本的占卜。"
command = "预测未来2"
mode = "image"
count = 0

[[shindan]]
id = "1124887"
title = "领养一只宠物吧"
description = """
😋😋😋
每日可领养一只
请对您的宠物负起责任来
👐👐👐"""
command = "领养宠物"
mode = "image"
count = 0

[[shindan]]
id = "55603"
title = "DQ呪文でおなにったーーーー"
description = "きがくるっとる"
command = "饲养宠物"
mode = "image"
count = 0

[[shindan]]
id = "556031"
title = "正确饲养你的方法！"
description = "怎样饲养宠物的你？（原作者不详，由Recital君重制）"
command = "饲养我"
mode = "image"
count = 0

[[shindan]]
id = "827891"
title = "今天拥有着什么样的香气呢？"
description = "今天的香气是什么呢？来闻闻吧。"
command = "香气"
mode = "image"
count = 0

[[shindan]]
id = "1126968"
title = "香水工坊"
description = "随机香水生成(简易版)，可能会有古怪的配方，后续会做个进阶版如果我想的起来的话(´ . .̫ . `)"
command = "香水"
mode = "image"
count = 0

[[shindan]]
id = "1059228"
title = "如果你是一隻馬娘"
description = "(試作)如果你是一隻馬娘"
command = "马娘"
mode = "image"
count = 0

[[shindan]]
id = "1059404"
title = "貴方と相性のいいウマ娘は？"
description = """
ウマ娘プリティーダービーのキャラの中から1人だけ選択されます。
[2022/05/19 更新] 8キャラ追加しました。
[2022/09/07 更新] 5キャラ追加しました。
[2023/04/21 更新] 12キャラ追加しました。
※現在107キャラ実装中"""
command = "马娘相性"
mode = "image"
count = 0

[[shindan]]
id = "1141852"
title = "你的骨设是什么"
description = """
生成自己的骨设，懒人必备；
如果有逻辑问题，就当它是个笑话吧；
可以作为参考设定。"""
command = "骨设"
mode = "image"
count = 0

[[shindan]]
id = "1093459"
title = "高考成绩生成器"
description = "卷！给爷使劲卷！"
command = "高考成绩"
mode = "image"
count = 0

[[shindan]]
id = "1157184"
title = "魂味人设生成器"
description = """
随机捏一个罐头
*有魂要素以及一些个人喜好作品的要素"""
command = "魂味人设"
mode = "image"
count = 0

[[shindan]]
id = "1113644"
title = "测一测你是什么类型的魅魔～♡"
description = "中文版魅魔生成器"
command = "魅魔"
mode = "image"
count = 0

[[shindan]]
id = "1148960"
title = "（自用）你的魔杖材料"
description = "你的魔杖是用什么做的"
command = "魔杖材料"
mode = "image"
count = 0

[[shindan]]
id = "940824"
title = "魔法人生：我在霍格沃兹读书时发生的两三事"
description = """
魔法人生：我在霍格沃兹读书时发生的两三事
占卜在魔法世界的经历。
作者微博：nickel拟科"""
command = "魔法人生"
mode = "image"
count = 0

[[shindan]]
id = "1203043"
title = "梦回四月药WEB ONLY-随机PARO摇摇乐"
description = """
【基于原作《魔法使的约定》制作的同人摇摇乐】本生成器会随机摇出两个PARO，可以在其中挑选想要创作的、或在CP向创作中让两人分属两个PARO，适用范围请自由发掘！
除了原作的愚人节PARO，还会出现各式各样的常见PARO。"""
command = "魔法使的约定同人"
mode = "image"
count = 0

[[shindan]]
id = "1147385"
title = "今天你的魔法少女穿什么？"
description = "顺手从扩展顺过来的随机表"
command = "魔法少女穿什么"
mode = "image"
count = 0

[[shindan]]
id = "1182814"
title = "（企划用）基础魔法属性测试"
description = "欢迎参加本次测试，过度紧张会影响身体数值，请放松心情，把手放到魔法水晶上。"
command = "魔法属性"
mode = "image"
count = 0

[[shindan]]
id = "913532"
title = "如果你在魔法世界里，你会拥有什么样的职业属性和魔法资质？"
description = "纯雪-155698号世界，这是个魔法为主的世界，可以说没有人不会魔法，只是使用的方法不太一样。当然，这个还是我自己用的人设随机器，还请各位随意。"
command = "魔法职业"
mode = "image"
count = 0

[[shindan]]
id = "426611"
title = "在魔法世界的能力或武器"
description = "你(妳)在魔法世界會擁有什麼能力或拿什麼武器呢?"
command = "魔法能力或武器"
mode = "image"
count = 0

[[shindan]]
id = "905558"
title = "你的魔物娘"
description = "有点色色的魔物娘人设生成器 每日变更√"
command = "魔物娘"
mode = "image"
count = 0

[[shindan]]
id = "1150319"
title = "【ハイスペックＡＩ】あなたが『魔王になったら』どんな感じなのか？"
description = "ステータスと魔王の画像を診断します！"
command = "魔王"
mode = "image"
count = 0

[[shindan]]
id = "749044"
title = "你作為魔王的事跡"
description = "你作為魔王的事跡"
command = "魔王事迹"
mode = "image"
count = 0

[[shindan]]
id = "686074"
title = "魔禁tf测试-能力测验结果咯"
description = "就是这样啦（beta）"
command = "魔禁"
mode = "image"
count = 0

[[shindan]]
id = "1212415"
title = "【鳴潮】恭喜你覺醒為共鳴者"
description = "快來看看你的角色介面吧☆"
command = "鸣潮角色"
mode = "image"
count = 0

[[shindan]]
id = "919572"
title = "小小的黄文元素生成器"
description = "嗯嗯嗯呃......用来自己写BL啥的总之是小黄文的时候使用的生成器......总之自己看（逃）"
command = "黄文元素"
mode = "image"
count = 0

[[shindan]]
id = "1121119"
title = "黄油角色测试（米拉版）"
description = "成分测试"
command = "黄油角色"
mode = "image"
count = 0

[[shindan]]
id = "1187589"
title = "每日黄色废料脑洞"
description = """
R18预警，可能含有BL/GB/背德元素
【有新了脑洞就会继续增添的！】
【下滑刷新即可随机下一条～】
（本来最初的计划是“一句话的黄色废料”，结果到现在越来越收不住手，已经变成小段子了！大家是更喜欢简短流还是小短文啊？要是喜欢长一些的，我就不收手了（ﾉ´д｀）
（目前在更隔壁另一个占卜）"""
command = "黄色废料"
mode = "image"
count = 0

[[shindan]]
id = "1115998"
title = "生成一个属于你的黑帮OC吧！内含特殊结果"
description = """
随便写写试试，有的长有的短，如果觉得生成的内容太短太少可能是有点不凑巧🤔换一个名字再试试吧…！只适合OC，不适合角色噢！
每个属性都隐藏了一条无厘头的特别内容😘结果每日改变！"""
command = "黑帮OC"
mode = "image"
count = 0

[[shindan]]
id = "1191558"
title = "【黑箱LSS】你和某个人困在了不XXX就不能出去的房间。"
description = """
你和黑箱LSS里的某个角色被困在一个房间里了 。
你们必须要xxx才能出去这个房间？！"""
command = "黑箱LSS"
mode = "image"
count = 0

[[shindan]]
id = "1204484"
title = "黑胶教团入教仪式"
description = """
（兽人男同向）
黑胶教团，一个掌握了永生与永恒快感的神秘教会，其教徒被一层致密的黑色乳胶从头到脚紧实地包裹覆盖，原本的样貌都被这层黑胶抹去，只有被黑胶勾勒的身型暗示着他们原来的种族。他们的男根无一例外地被囚禁在由黑胶形成的圆形锁包之中，锁包上的锁状图案昭示着他们永无释放之日的命运，也蕴含着永恒快感的神秘。
你因为一些原因加入或被加入了黑胶教团！看看教团里的你是什么样子吧！"""
command = "黑胶教团入教仪式"
mode = "image"
count = 0

[[shindan]]
id = "1007603"
title = "屬於你的龍與地下城(DND)冒險者之證"
description = """
基於5E版本製成，因全亂數組成不建議用於正常跑團。
亦可單純視為中世紀奇幻風格的人物速建。

※屬性下限為3，上限為18。
※普通人類的各項平均數值為10~11。"""
command = "龙与地下城"
mode = "image"
count = 0
"###;
