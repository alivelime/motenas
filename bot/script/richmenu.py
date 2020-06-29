#!/usr/bin/env python3

# pip install line-bot-sdk
# pip install pyyaml
# pip install boto3
import sys
import requests
import yaml
import boto3
from linebot import (
    LineBotApi
)
from linebot.models import (
    RichMenu, RichMenuArea, RichMenuSize, RichMenuBounds,
    PostbackAction, URIAction,
)


def main():
    if len(sys.argv) <= 2:
        print('script/richmenu.py dev TokishirazuLlcPassengersBachan')
        return 

    env = sys.argv[1]
    chara = sys.argv[2]

    with open('conf/line-'+env+'.yml') as file:
        yml = yaml.safe_load(file)

    chara_name = yml[chara+'_CHARA_ID']
    omise_id = yml[chara+'_OMISE_NAME']
    order_group_id = yml[omise_id+'_ORDER_GROUP_ID']
    access_token = yml[chara+'_CHANNEL_TOKEN']
    # dynamodb = boto3.resource('dynamodb', region_name='ap-northeast-1')
    # table = dynamodb.Table(env+'omise')

    if env == "dev":
        LIFF_HOST = 'https://liff.line.me/1654294151-W4nvdn0p/dev'
    elif env == 'prd':
        LIFF_HOST = 'https://liff.line.me/1654396683-AKjozgVr/prd'
    else:
        LIFF_HOST = ''


    line_bot_api = LineBotApi(access_token)

    line_bot_api.cancel_default_rich_menu()

    delete_richmenu(line_bot_api)
    id = create_user_richmenu(LIFF_HOST, line_bot_api, chara_name)
    upload_user_richmenu_image(line_bot_api, id)

    line_bot_api.set_default_rich_menu(id)
    
    # メインキャラじゃないならここでリターン
    return 
    
    id = create_staff_richmenu(LIFF_HOST, line_bot_api, chara_name)
    upload_staff_richmenu_image(line_bot_api, id)

    set_staff_richmenu(line_bot_api, id )

    return

def set_staff_richmenu(line_bot_api, id):
    # line_bot_api.link_rich_menu_to_user('', id)
    line_bot_api.link_rich_menu_to_users([
        'Ube2da389e4c223405286c03a32fefcb6',
        # 'U15022dc52eb46f56a4ea7d7ee3fcaebe',
    ], id)
    return


def delete_richmenu(line_bot_api):
    print("delete user richmenu")
    menu_list = line_bot_api.get_rich_menu_list()

    for richmenu in menu_list:
        print("delete user richmenu "+richmenu.rich_menu_id)
        line_bot_api.delete_rich_menu(richmenu.rich_menu_id)
        
    
                    
def create_user_richmenu(LIFF_HOST, line_bot_api, chara_name):
    print("create user richmenu")
    user_menu = RichMenu(
        size=RichMenuSize(width=1200, height=405),
        selected=True,
        name="minarai-chan user menu",
        chat_bar_text="お店情報・ご注文",
        areas=[
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=0,
                    y=0,
                    width=400,
                    height=405,
                ),
                action=PostbackAction(
                    label='何ができるの?',
                    displayText='何ができるの?',
                    data="how_to",
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=400,
                    y=0,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='omise',
                    uri=LIFF_HOST+'/user/omise/'+chara_name,
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=800,
                    y=0,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='order',
                    uri=LIFF_HOST+'/user/order/'+chara_name,
                ),
            ),
        ]
    )
    id = line_bot_api.create_rich_menu(rich_menu=user_menu)
    print("user richmenu id = "+id)
    return id

def upload_user_richmenu_image(line_bot_api, id):
    print("update user richmenu "+ id)
    with open('script/image/richmenu/user.png', 'rb') as f:
        line_bot_api.set_rich_menu_image(id, 'image/png', f)

def create_staff_richmenu(LIFF_HOST, line_bot_api, chara_name):
    print("create staff richmenu")
    user_menu = RichMenu(
        size=RichMenuSize(width=1200, height=810),
        selected=False,
        name="minarai-chan user menu",
        chat_bar_text="管理メニュー",
        areas=[
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=0,
                    y=0,
                    width=400,
                    height=405,
                ),
                action=PostbackAction(
                    label='何ができるの?',
                    displayText='何ができるの?',
                    data="how_to_omise",
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=400,
                    y=0,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='omise',
                    uri=LIFF_HOST+'/user/omise/'+chara_name,
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=800,
                    y=0,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='order',
                    uri=LIFF_HOST+'/user/order/'+chara_name,
                ),
            ),
            # 下の段
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=0,
                    y=405,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='staff_maroudo',
                    uri=LIFF_HOST+'/staff/maroudo/'+chara_name,
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=400,
                    y=405,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='staff_omise',
                    uri=LIFF_HOST+'/staff/omise/'+chara_name,
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=800,
                    y=405,
                    width=400,
                    height=405,
                ),
                action=URIAction(
                    label='staff_order',
                    uri=LIFF_HOST+'/staff/order/'+chara_name,
                ),
            ),
        ]
    )
    id = line_bot_api.create_rich_menu(rich_menu=user_menu)
    print("omise richmenu id = "+id)
    return id

def upload_staff_richmenu_image(line_bot_api, id):
    print("update staff richmenu "+ id)
    with open('script/image/richmenu/omise.png', 'rb') as f:
        line_bot_api.set_rich_menu_image(id, 'image/png', f)

main()
