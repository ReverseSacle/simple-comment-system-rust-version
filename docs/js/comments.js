/* 服务器地址 */
const _host = "192.168.80.150";

/* 前端POST请求 */
async function _PostRequest(url,data)
{
    try{
        const _URL = "http://" + _host + url;
        var response = await fetch(
            _URL,
            {
                method: "POST",
                headers:{
                    "Content-Type": "text/plain",
					"Content-length": data.length
                },
                body: data,
            }
        );
        return response.ok;
    } catch(error) { return false; }
}

/* 前端GET请求 */
async function _GetRequest(url)
{
    try{
        const _URL = "http://" + _host + url;
        var response = await fetch(_URL);

        if(!response.ok){ return [false,""]; }

        return [true,await response.text()];
    } catch(error) { return [false,""]; }
}

/* 获取主线评论的数量 */
async function commentCount()
{
    const responseData = await _GetRequest("/database?count");
    var count = 0;

    if(responseData[0]){ count = responseData[1]; }

    return count;
}

/**  
 * 评论内容的文本的美化 
 * 主要是将空格类字符去除，再根据自定义的`多少字符一行`规则来添加换行标签
**/ 
function textareaBeautify(textsLabel,cols)
{
    var cache = "";
    var texts = (textsLabel.value).replace(/\r|\n/g,"");
    var textLength = texts.length;

    for(var i = 0;i < textLength;++i)
    {
        if(0 != cache.length && 0 === cache.length % cols){
            cache += "<br/>";
        }
        cache += texts[i];
    };
    return cache;
}

/**
 * 时间戳转换为格式化时间 
 * 将时间戳与当前时间的差值算出，
 * 再按天，时，分的顺序算，
 * 时，中会减去经过的天数时间，
 * 分，中会减去经的天数时间的总分钟与计算后时的总分钟。
 * 展示时优先按天，时，分的顺序显示，从最前位开始，不为零就显示。
 * 当天数超过365天后就直接按`年-月-日`的格式显示。
**/ 
function timestampTackle(timeSpendLabel,create_at)
{
    const timeSpace = Date.now() - create_at;
    const days = timeSpace / 1000 / 60 / 60 / 24;
    const f_days = Math.floor(days);
    const hours = timeSpace / 1000 / 60 / 60 - (24 * f_days);
    const f_hours = Math.floor(hours);
    const minutes = timeSpace / 1000 / 60 - (24 * 60 * f_days) - (60 * f_hours);
    const f_minutes = Math.floor(minutes);

    if(0 != f_days)
    {
        if(f_days > 365)
        {
            const date = new Date(timeSpace);
            const year = date.getFullYear();
            const month = (date.getMonth() + 1).toString().padStart(2,'0');
            const day = date.getDate().toString().padStart(2,'0');
            const format_time = `${year}-${month}-${day}`;
            timeSpendLabel.innerText = "" + format_time;
        } else { timeSpendLabel.innerText = "" + f_days + " 天前"; }
    }
    else if(0 != f_hours){ timeSpendLabel.innerText = "" + f_hours + " 小时前"; }
    else{ timeSpendLabel.innerText = "" + f_minutes + " 分钟前"; }
}

/**
 * 评论标签的头像处理 
 * 适配了Gravatar API
**/ 
function avatarAppend(mainUserLabel,mail_md5)
{
    var avatarLabel = document.createElement("div");
    avatarLabel.setAttribute("class","avatar");

    var imgUrl = "https://gravatar.com/avatar/";
    avatarLabel.innerHTML = "<img src=\"" + imgUrl + mail_md5 + "\" alt=img>";
    mainUserLabel.appendChild(avatarLabel);
}

/** 
 * 评论框中评论内容的添加 
 * 就将从数据库表中获取的内容放到html的评论标签中
**/ 
function commentAppend(mainUserLabel,record,response_label = null)
{
    var commentBoxLabel = document.createElement("div");
    commentBoxLabel.setAttribute("class","comment-box");

    /* comment box head area star-area */
    /****************************************************/
    var boxHead = document.createElement("div");
    boxHead.setAttribute("class","cbox-head clearfix");

    /* nickname area */
    var authorName = document.createElement("span");
    authorName.setAttribute("class","cbauthor-name");
    authorName.innerText = record.nick_name;

    /* timestamp transform into format time */
    var timeSpendLabel = document.createElement("span");
    timestampTackle(timeSpendLabel,record.create_at);

    /* reply area */
    var replyLabel = document.createElement("div");
    replyLabel.setAttribute("class","reply");
    replyLabel.setAttribute("response_id",record.id);
    replyLabel.innerText = "回复";

    boxHead.appendChild(authorName);
    boxHead.appendChild(timeSpendLabel);
    boxHead.appendChild(replyLabel);
    commentBoxLabel.appendChild(boxHead);
    /****************************************************/
    /* comment box head area end-area */

    /* coment box content area start-area */
    var boxContent = document.createElement("span");
    boxContent.setAttribute("class","cbox-content");

    var response_user = "";
    if(null != response_label)
    {
        response_user = (
            "<a class=\"at\" href=\"#\">" + 
            "@" + response_label.getAttribute("nick_name") + 
            "</a>" + ","
        );
    }
    boxContent.innerHTML = (
        response_user + 
        "<p>" + 
        record.comment +
        "</p>"
    );
    commentBoxLabel.appendChild(boxContent);

    mainUserLabel.appendChild(commentBoxLabel);
}

/**
 * 评论标签的样式创建以及内容添加
 * 新的评论标签的样式和内容添加，
 * 以及通过控制评论展示区css样式的高度属性来让展示区域实现变动
**/ 
function contentAppend(userBlock,record,response_label = null)
{
    var commentBlock = document.getElementById("comment-block");
    var commentContainer = commentBlock.lastElementChild;
    var mainUserLabel = document.createElement("div");

    mainUserLabel.setAttribute("class","main-level clearfix");

    avatarAppend(mainUserLabel,record.mail_md5);
    commentAppend(mainUserLabel,record,response_label);
    userBlock.appendChild(mainUserLabel);

    /* state stretch */
    var CB_Height = commentBlock.clientHeight;
    var newHeight = CB_Height + 90;

    commentBlock.style.height = newHeight + "px";
    commentContainer.style.height = newHeight - 325 +  "px";
}

/** 
 * 回复按钮的输入区域的右上角的X的点击功能
 * 就是将由评论按钮功能产生的评论输入区标签删减，
 * 接着再调整发生高度变化的评论展示区和当前评论的内容展示区的高度到原来的样子
**/ 
function replyCancleClick(reply_label)
{
    var cboxHead = reply_label.parentElement;
    var commentBox = cboxHead.parentElement;
    var cboxContent = cboxHead.nextElementSibling;
    var commentBlock = document.getElementById("comment-block");
    var CB_height = commentBlock.clientHeight;
    var CC_height = cboxContent.clientHeight;

    commentBlock.style.height = CB_height - 305 + "px";
    cboxContent.style.height = CC_height - 340 + "px";
    cboxContent.style.display = "unset";

    commentBox.removeChild(commentBox.lastElementChild);	
}

/** 
 * 回复按钮的输入区域的右小角的提交的点击功能
 * 与评论输入区域的提交按钮相比，回复按钮的评论输入区的提交按钮多了
 * 父评论的查找，以及提交后，回复评论的输入区会消失
**/ 
async function replyCommitClick(reply_label)
{
    var cboxHead = reply_label.parentElement;
    var cboxContent = cboxHead.nextElementSibling;
    var inputBlock = cboxContent.nextElementSibling;
    var inputArea = inputBlock.firstElementChild;
    var nickname_label = inputArea.firstElementChild;
    var email_label = nickname_label.nextElementSibling;
    var button_label = email_label.nextElementSibling;
    var text_label = button_label.nextElementSibling;

    var nickname_label_value = nickname_label.value;
    var email_md5 = CryptoJS.SHA256(email_label.value);
    var reply_parent = cboxHead;

    while("parent_id" != reply_parent.getAttribute("class")){
        reply_parent = reply_parent.parentElement;
    }

    var response_id = reply_label.getAttribute("response_id");
    var parent_id = reply_parent.getAttribute("id");

    if(0 != nickname_label_value.length)
    {
        var comment = textareaBeautify(
            text_label,
            text_label.cols
        );
        var curTime = Date.now();
        var data = (
            ":" +
            parent_id + ":" +
            response_id + ":" +
            nickname_label_value + ":" +
            email_label.value + ":" +
            email_md5 + ":" +
            curTime + ":" +
            comment
        );

        if(true == await _PostRequest("/database?insert",data))
        {
            replyCancleClick(reply_label)
            commentReplyShow();
        }
    }
}

/** 
 * 回复按钮的点击功能
 * 就是在当前所点击的评论的标签下添加一个评论输入区，
 * 额外多了个`x`撤销按钮，以及提交按钮的功能发生了变化
**/ 
function replyClick(reply_label)
{
    var cboxHead = reply_label.parentElement;
    var reply_commentBox = cboxHead.parentElement;
    var comment_inputBlock = reply_commentBox.lastElementChild;

    if("input-block" == comment_inputBlock.getAttribute("class")){
        return;
    }

    var commentBlock = document.getElementById("comment-block");
    var reply_inputBlock = document.createElement("div");
    var reply_inputArea = document.createElement("div");
    var reply_nickname = document.createElement("input");
    var reply_email = document.createElement("input");
    var reply_textBlock = document.createElement("textarea");
    var reply_buttonArea = document.createElement("div");
    var reply_commit = document.createElement("div");

    reply_inputBlock.setAttribute("class","input-block");
    reply_inputArea.setAttribute("class","input-area clearfix");

    reply_nickname.setAttribute("type","text");
    reply_nickname.setAttribute("class","nickname");
    reply_nickname.setAttribute("maxlength","19");
    reply_nickname.setAttribute("placeholder"," nickname(1~19个字符)");

    reply_email.setAttribute("type","email");
    reply_email.setAttribute("class","email");
    reply_email.setAttribute("maxlength","44");
    reply_email.setAttribute("placeholder"," email(8~44个字符)");

    reply_textBlock.setAttribute("class","text-block");
    reply_textBlock.setAttribute("cols","50");
    reply_textBlock.setAttribute("placeholder"," ");

    reply_buttonArea.setAttribute("class","button-area clearfix");
    reply_commit.setAttribute("type","button");
    reply_commit.setAttribute("class","commit");
    reply_commit.innerText = "提交";

    reply_inputBlock.appendChild(reply_inputArea);
    reply_inputArea.appendChild(reply_nickname);
    reply_inputArea.appendChild(reply_email);
    reply_inputArea.appendChild(reply_textBlock);
    reply_inputArea.appendChild(reply_buttonArea);

    reply_buttonArea.appendChild(reply_commit);
    reply_commentBox.appendChild(reply_inputBlock);

    var origin_height = commentBlock.clientHeight;
    commentBlock.style.height = origin_height + 300 + "px";

    var reply_prev_sibling = reply_inputBlock.previousSibling;
    var orgin_prev_height = reply_prev_sibling.clientHeight;
    reply_prev_sibling.style.height = orgin_prev_height + 335 + "px";
    reply_prev_sibling.style.display = "block";

    var reply_cancle = document.createElement("div");
    reply_cancle.setAttribute("type","button");

    reply_cancle.setAttribute("class","cancle");
    reply_cancle.innerText = "X";
    reply_inputArea.insertBefore(reply_cancle,reply_textBlock);

    reply_cancle.addEventListener(
        "click",
        function(){ replyCancleClick(reply_label); },
        false
    );

    reply_commit.addEventListener(
        "click",
        function(){ replyCommitClick(reply_label); },
        false
    );
}

/** 
 * 评论展示区的加载
 * 向数据库请求主线评论数据，
 * 接着将请求的内容按照html标签的所属关系依次建立并将内容添加上去
**/ 
async function commentPreShow()
{
    const responseData = await _GetRequest("/database?parent");
    if(false == responseData[0]){ return; }

    var data = responseData[1];
    var array = data.split(",");
    const array_length = array.length;

    if(0 == array_length || 0 == array_length - 1){
        return;
    }

    var commentBlock = document.getElementById("comment-block");
    var commentContainer = commentBlock.lastElementChild;

    if("comment-container" != commentContainer.getAttribute("class"))
    {
        var newCommentContainer = document.createElement("div");
        var newInfo = document.createElement("span");
        var newCommentList = document.createElement("ul");

        newCommentContainer.setAttribute("class","comment-container");
        newInfo.setAttribute("class","info");
        newInfo.setAttribute("type","text");
        newCommentList.setAttribute("class","comment-list");

        newCommentContainer.appendChild(newInfo);
        newCommentContainer.appendChild(newCommentList);
        commentBlock.appendChild(newCommentContainer);

        commentContainer = newCommentContainer;
    }
    var commentList = commentContainer.lastElementChild;
    var record = new Object();

    array.pop();
    for(var i = 0;i < array_length;++i)
    {
        var sub = array[i];
        var member_ptr = 0;

        record.id = "";
        record.parent_id = "";
        record.response_id = "";
        record.nick_name = "";
        record.mail = "";
        record.mail_md5 = "";
        record.create_at = "";
        record.comment = "";

        if(undefined == sub){ continue; }
        for(var j = 0;j < sub.length;++j)
        {
            const c = sub[j];

            if(":" == c && 7 != member_ptr){ ++member_ptr; }
            else
            {
                switch(member_ptr)
                {
                    case 0: { record.id += c; break; }
                    case 1: { record.parent_id += c; break; }
                    case 2: { record.response_id += c; break; }
                    case 3: { record.nick_name += c; break; }
                    case 4: { record.mail += c; break; }
                    case 5: { record.mail_md5 += c; break; }
                    case 6: { record.create_at += c; break; }
                    case 7: { record.comment += c; break; }
                }
            }
        }
        var userBlock = document.createElement("li");
        var commentListFirstChild = commentList.firstElementChild;

        userBlock.setAttribute("id",record.id);
        userBlock.setAttribute("nick_name",record.nick_name);
        userBlock.setAttribute("class","parent_id");	

        contentAppend(userBlock,record);
        if(null == commentListFirstChild)
        {
            commentList.appendChild(userBlock);
            commentList.style.display = "block";
        }
        else{ commentList.insertBefore(userBlock,commentListFirstChild); }
    }
    var commentNum = Number(await commentCount());
    var info = commentContainer.firstElementChild;
    var replyElements = commentList.getElementsByClassName("reply");

    info.innerText = "已有" + commentNum + "条评论";

    for(const reply of replyElements)
    {
        reply.addEventListener(
            "click",
            function(){ replyClick(reply); },
            false
        );
    }
	commentReplyShow();
}

/** 
 * 评论输入区域的右下角的点击功能
 * 就是将输入区输入的内容按照前后端规定的统一数据格式，发送给后端，
 * 成功发送后，评论展示区的样式会发生变化
**/ 
async function buttonClick()
{
    var commentBlock = document.getElementById("comment-block");
    var commentContainer = commentBlock.lastElementChild;

    if("comment-container" != commentContainer.getAttribute("class"))
    {
        var newCommentContainer = document.createElement("div");
        var newInfo = document.createElement("span");
        var newCommentList = document.createElement("ul");

        newCommentContainer.setAttribute("class","comment-container");
        newInfo.setAttribute("class","info");
        newInfo.setAttribute("type","text");
        newCommentList.setAttribute("class","comment-list");

        newCommentContainer.appendChild(newInfo);
        newCommentContainer.appendChild(newCommentList);
        commentBlock.appendChild(newCommentContainer);

        commentContainer = newCommentContainer;
    }
    var commentList = commentContainer.lastElementChild;
    var inputBlock = commentBlock.firstElementChild;
    var inputArea = inputBlock.firstElementChild;

    var nickname_label = inputArea.firstElementChild;
    var email_label = nickname_label.nextElementSibling;
    var text_label = email_label.nextElementSibling;
    var email_md5 = CryptoJS.SHA256(email_label.value);	
    var nickname_label_value = nickname_label.value;

    if(0 != nickname_label_value.length)
    {
        /* current time */
        var comment = textareaBeautify(text_label,text_label.cols);
        var curTime = Date.now();
        var data = (
            ":" + 
            ":" + 
            ":" + 
            nickname_label_value + ":" + 
            email_label.value + ":" + 
            email_md5 + ":" + 
            curTime  + ":" + 
            comment
        );

        if(true == await _PostRequest("/database?insert",data))
        {
            commentContainer.removeChild(commentList);
            commentBlock.style.height = "360px";
            commentContainer.style.height = "0px";

            var commentList = document.createElement("ul");
            var commentNum = Number(await commentCount()) + 1;
            var info = commentContainer.firstElementChild;

            commentList.setAttribute("class","comment-list");
            commentContainer.appendChild(commentList);
            commentPreShow();

            info.innerText = "已有" + commentNum + "条评论";
        }
    }

    nickname_label.value = "";
    email_label.value = "";
    text_label.value = "";
}


/** 
 * 评论展示区中主线子评论的加载 
 * 由于之前显示的子评论会造成重复现象，故需先将所用主线评论中的主线子评论列表删除，
 * 之后从后端数据库中获取主线子评论，
 * 依次按照获取的记录中的parent_id来将主线子评论，添加到对应的主线评论下面
*/
async function commentReplyShow()
{
    const responseData = await _GetRequest("/database?reply");
    if(false == responseData[0]){ return; }

    var data = responseData[1];
    var array = data.split(",");
    const array_length = array.length;

    if(0 == array_length || 0 == array_length - 1){
        return;
    }

    var commentBlock = document.getElementById("comment-block");
    var parent_id_elements = commentBlock.getElementsByClassName("parent_id");
    var record = new Object();

    for(const p of parent_id_elements)
    {
        var child_element = p.lastElementChild;
        if("child-list" == child_element.getAttribute("class")){
            p.removeChild(child_element);
        }
    }

    array.pop();
    for(var i = 0;i < array_length;++i)
    {
        var sub = array[i];
        var member_ptr = 0;

        record.id = "";
        record.parent_id = "";
        record.response_id = "";
        record.nick_name = "";
        record.mail = "";
        record.mail_md5 = "";
        record.create_at = "";
        record.comment = "";

        if(undefined == sub){ continue; }
        for(var j = 0;j < sub.length;++j)
        {
            const c = sub[j];

            if(":" == c && 7 != member_ptr){ ++member_ptr; }
            else
            {
                switch(member_ptr)
                {
                    case 0: { record.id += c; break; }
                    case 1: { record.parent_id += c; break; }
                    case 2: { record.response_id += c; break; }
                    case 3: { record.nick_name += c; break; }
                    case 4: { record.mail += c; break; }
                    case 5: { record.mail_md5 += c; break; }
                    case 6: { record.create_at += c; break; }
                    case 7: { record.comment += c; break; }
                }
            }
        }
        var parent_label = document.getElementById(record.parent_id);
        var response_label = document.getElementById(record.response_id);
        var childList = parent_label.lastElementChild;
        var userBlock = document.createElement("li");

        if("child-list" != childList.getAttribute("class"))
        {
            childList = document.createElement("ul");
            childList.setAttribute("class","child-list");
            childList.style.marginTop = "5px";

            parent_label.appendChild(childList);
        }

        userBlock.setAttribute("id",record.id);
        userBlock.setAttribute("class","child_id");
        userBlock.setAttribute("nick_name",record.nick_name);
        userBlock.setAttribute("parent_id",record.parent_id);
        userBlock.setAttribute("response_id",record.response_id);

        contentAppend(userBlock,record,response_label);
        if(null == childList.firstChild)
        {
            childList.appendChild(userBlock);
            childList.style.display = "block";
        } else { childList.appendChild(userBlock); }
    }

    if(null != parent_id_elements)
    {
        var commentContainer = commentBlock.lastElementChild;
        var commentList = commentContainer.lastElementChild;
        var replyElements = commentList.getElementsByClassName("reply");

        for(const reply of replyElements)
        {
            reply.addEventListener(
                "click",
                function(){ replyClick(reply); },
                false
            );
        }
    }
}

/* 输入http链接时客户端加载响应函数 */
window.onload = async function(){
    var commentBlock = document.getElementById("comment-block");
    var inputBlock = commentBlock.firstElementChild;
    var inputArea = inputBlock.firstElementChild;
    var buttonArea = inputArea.lastElementChild;
    var commitButton = buttonArea.firstElementChild;

    await commentPreShow();
    commitButton.addEventListener(
        "click",
        function(){ buttonClick(); },
        false
    );
};
